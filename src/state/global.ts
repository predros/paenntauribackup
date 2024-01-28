import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { LogicalPosition, appWindow } from "@tauri-apps/api/window";
import {
  ClickType,
  type ICombination,
  type ILoadcase,
  type IMaterial,
  type IMemberResult,
  type ISection,
  type ReactionsDict,
  ResultType,
  type ResultsDict,
  type SectionType,
  SideBarType,
} from "@/types/types";

import useNodeStore from "@/state/nodes";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";

import { type IPoint, type IRectangle } from "@/helper/math";
import { useI18n } from "vue-i18n";
import { ask, open, save } from "@tauri-apps/api/dialog";
import { exit } from "@tauri-apps/api/process";

export default defineStore("global", () => {
  const { t } = useI18n();

  const nodes = useNodeStore();
  const members = useMemberStore();
  const settings = useSettings();

  const loadcasesList = ref<ILoadcase[]>([]);
  const combinationsList = ref<ICombination[]>([]);
  const materialsList = ref<IMaterial[]>([]);
  const sectionsList = ref<ISection[]>([]);

  const canvasProps = ref({
    scale: 1.0,
    mousePosition: null as IPoint | null,
    showGrid: true,
    gridSnap: false,
    newMemberAnchor: null as IPoint | null,
    viewPortBounds: {
      x: 0,
      y: 0,
      width: 0,
      height: 0,
    } as IRectangle,
  });

  const dialogs = ref({
    alert: {
      show: false,
      text: "",
    },
    runningAnalysis: false,
    loadcases: false,
  });

  const current = ref({
    clickType: ClickType.Select,
    sideBarType: SideBarType.Select,
    result: {
      isCombination: false, // Combinations haven't been implemented yet
      id: 0,
    },
    loadcase: 0,
    material: null as number | null,
    section: null as number | null,
    selected: {
      nodes: [] as number[],
      members: [] as number[],
    },
  });

  const results = ref({
    type: ResultType.Displacement,
    scale: 1.0,
    showReactions: true,
    loadcases: {
      members: null as ResultsDict | null,
      reactions: null as ReactionsDict | null,
    },
    combinations: {
      members: null as ResultsDict | null,
      reactions: null as ReactionsDict | null,
    },
    selected: {
      id: null as number | null,
      position: null as number | null,
    },
  });

  const historyLength = ref({ undo: 0, redo: 0 });

  const resultsExtrema = computed(() => {
    const result = {
      moment: 0,
      normal: 0,
      shear: 0,
    };

    let currentCase: IMemberResult[];
    const currentResult = current.value.result;

    if (currentResult.isCombination) {
      if (results.value.combinations.members === null) {
        return result;
      }
      currentCase = results.value.combinations.members[currentResult.id];
    } else {
      if (results.value.loadcases.members === null) {
        return result;
      }
      currentCase = results.value.loadcases.members[currentResult.id];
    }

    currentCase.forEach((member) => {
      const momentFirst = member.moment.at(0);
      const momentLast = member.moment.at(-1);
      if (momentFirst != undefined) {
        result.moment = Math.max(result.moment, Math.abs(momentFirst));
      }
      if (momentLast != undefined) {
        result.moment = Math.max(result.moment, Math.abs(momentLast));
      }
      result.moment = Math.max(
        result.moment,
        Math.abs(member.maxMoment[1]),
        Math.abs(member.minMoment[1]),
      );

      const shearFirst = member.shear.at(0);
      const shearLast = member.shear.at(-1);
      if (shearFirst != undefined) {
        result.shear = Math.max(result.shear, Math.abs(shearFirst));
      }
      if (shearLast != undefined) {
        result.shear = Math.max(result.shear, Math.abs(shearLast));
      }
      result.shear = Math.max(result.shear, Math.abs(member.vertShear[1]));

      const normalFirst = member.normal.at(0);
      const normalLast = member.normal.at(-1);
      if (normalFirst != undefined) {
        result.normal = Math.max(result.normal, Math.abs(normalFirst));
      }
      if (normalLast != undefined) {
        result.normal = Math.max(result.normal, Math.abs(normalLast));
      }
      result.normal = Math.max(result.normal, Math.abs(member.vertNormal[1]));
    });

    result.moment = result.moment != 0 ? result.moment : 1;
    result.shear = result.shear != 0 ? result.shear : 1;
    result.normal = result.normal != 0 ? result.normal : 1;

    return result;
  });

  async function analysisRun(): Promise<void> {
    dialogs.value.runningAnalysis = true;
    try {
      const response = await invoke("analysis_run_linear").catch((e) => {
        appAlert(e);
      });
      const result = response as [
        ResultsDict,
        ReactionsDict,
        ResultsDict,
        ReactionsDict,
      ];

      select([], []);
      results.value.loadcases.members = result[0];
      results.value.loadcases.reactions = result[1];

      results.value.combinations.members = result[2];
      results.value.combinations.reactions = result[3];

      results.value.selected.id = null;
      results.value.selected.position = null;

      current.value.clickType = ClickType.Result;
      current.value.sideBarType = SideBarType.Result;
    } catch (error) {
      const e = error as string[];
      appAlert(t(e[0], e[1]));
    } finally {
      dialogs.value.runningAnalysis = false;
    }
  }

  function appAlert(text: string): void {
    dialogs.value.alert.text = text;
    dialogs.value.alert.show = true;
  }

  async function appCursorSnap(windowPos: IPoint): Promise<void> {
    await appWindow
      .setCursorPosition(new LogicalPosition(windowPos.x, windowPos.y))
      .catch((e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      });
  }

  async function appExit(): Promise<void> {
    const fileChanged = await invoke("file_unsaved_changes");
    if (fileChanged) {
      const unsavedDiscard = await ask(t("dialogs.native.unsavedChanges"), {
        title: t("dialogs.native.unsavedChangesTitle"),
        type: "warning",
        okLabel: t("dialogs.native.buttons.yes"),
        cancelLabel: t("dialogs.native.buttons.no"),
      });

      if (!unsavedDiscard) {
        return;
      }
    }

    await exit(0);
  }

  async function appRedo(): Promise<void> {
    const result = await invoke("app_redo").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function appUndo(): Promise<void> {
    const result = await invoke("app_undo").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function combinationApplyFactors(
    factors: Record<number, Record<number, number>>,
  ): Promise<void> {
    await invoke("combination_apply_factors", { factors }).catch(
      (e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      },
    );

    await fetchCombinations();
  }

  async function combinationDelete(id: number): Promise<void> {
    await invoke("combination_delete", { id }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    await fetchLoadcases();
    await fetchCombinations();
  }

  async function combinationNew(name: string): Promise<void> {
    await invoke("combination_new", { name }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    await fetchLoadcases();
    await fetchCombinations();
  }

  async function combinationUpdate(id: number, name: string): Promise<void> {
    await invoke("combination_update", { id, name }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    await fetchLoadcases();
    await fetchCombinations();
  }

  async function fetchCombinations(): Promise<void> {
    const result = await invoke("combination_get_dtos").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    combinationsList.value = result as ICombination[];
  }

  async function fetchEverything(): Promise<void> {
    await settings.fetchSettings();

    await fetchMaterials();
    await fetchSections();
    await nodes.fetchNodes();
    await members.fetchMembers();

    await fetchLoadcases();
    await fetchCombinations();
    await fetchLoadcaseCurrent();
  }

  async function fetchLoadcaseCurrent(): Promise<void> {
    const id = await invoke("loadcase_get_current").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    current.value.loadcase = id as number;
  }

  async function fetchLoadcases(): Promise<void> {
    const result = await invoke("loadcase_get_dtos").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    loadcasesList.value = result as ILoadcase[];
  }

  async function fetchMaterials(): Promise<void> {
    const result = await invoke("material_get_dtos").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    materialsList.value = result as IMaterial[];
  }

  async function fetchSections(): Promise<void> {
    const result = await invoke("section_get_dtos").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    sectionsList.value = result as ISection[];
  }

  async function fileNew(): Promise<void> {
    const fileChanged = await invoke("file_unsaved_changes");

    if (fileChanged) {
      const unsavedDiscard = await ask(t("dialogs.native.unsavedChanges"), {
        title: t("dialogs.native.unsavedChangesTitle"),
        type: "warning",
        okLabel: t("dialogs.native.buttons.yes"),
        cancelLabel: t("dialogs.native.buttons.no"),
      });

      if (!unsavedDiscard) {
        return;
      }
    }

    const result = await invoke("file_new").catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await fetchEverything();
    current.value.sideBarType = SideBarType.Select;
    current.value.clickType = ClickType.Select;
  }

  async function fileOpen(): Promise<void> {
    const fileChanged = await invoke("file_unsaved_changes");

    if (fileChanged) {
      const unsavedDiscard = await ask(t("dialogs.native.unsavedChanges"), {
        title: t("dialogs.native.unsavedChangesTitle"),
        type: "warning",
        okLabel: t("dialogs.native.buttons.yes"),
        cancelLabel: t("dialogs.native.buttons.no"),
      });

      if (!unsavedDiscard) {
        return;
      }
    }

    const selectedFile = await open({
      title: t("dialogs.native.fileOpenTitle"),
      multiple: false,
      filters: [
        {
          name: t("dialogs.native.fileType"),
          extensions: ["pnn"],
        },
      ],
    });

    if (selectedFile === null || Array.isArray(selectedFile)) {
      return;
    }

    const result = await invoke("file_open", { path: selectedFile }).catch(
      (e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      },
    );

    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await fetchEverything();
    current.value.sideBarType = SideBarType.Select;
    current.value.clickType = ClickType.Select;
  }

  async function fileSave(): Promise<void> {
    const currentFile = (await invoke("file_get_current")) as string;

    let savePath: string | null;
    if (currentFile.trim() == "") {
      savePath = await save({
        title: t("dialogs.native.fileSaveTitle"),
        filters: [
          {
            name: t("dialogs.native.fileType"),
            extensions: ["pnn"],
          },
        ],
      });
    } else {
      savePath = currentFile;
    }
    if (savePath === null) {
      return;
    }

    await invoke("file_save", { path: savePath }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
  }

  async function fileSaveAs(): Promise<void> {
    const savePath = await save({
      title: t("dialogs.native.fileSaveTitle"),
      filters: [
        {
          name: t("dialogs.native.fileType"),
          extensions: ["pnn"],
        },
      ],
    });
    if (savePath === null) {
      return;
    }

    await invoke("file_save", { path: savePath }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
  }

  async function loadcaseChangeCurrent(newId: number): Promise<void> {
    await invoke("loadcase_set_current", { id: newId }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function loadcaseDelete(id: number): Promise<void> {
    await invoke("loadcase_delete", { id }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    await fetchLoadcases();
    await fetchCombinations();
  }

  async function loadcaseNew(name: string): Promise<void> {
    await invoke("loadcase_new", { name }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    await fetchLoadcases();
    await fetchCombinations();
  }

  async function loadcaseUpdate(id: number, name: string): Promise<void> {
    await invoke("loadcase_update", { id, name }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    await fetchLoadcases();
    await fetchCombinations();
  }

  async function materialDelete(id: number): Promise<void> {
    await invoke("material_delete", { id }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    await fetchMaterials();
  }

  async function materialNew(
    name: string,
    elasticity: number,
    thermal: number,
  ): Promise<void> {
    await invoke("material_new", { name, elasticity, thermal }).catch(
      (e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      },
    );
    await fetchMaterials();
  }

  async function materialUpdate(
    id: number,
    name: string,
    elasticity: number,
    thermal: number,
  ): Promise<void> {
    await invoke("material_update", { id, name, elasticity, thermal }).catch(
      (e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      },
    );
    await fetchMaterials();
  }

  async function sectionDelete(id: number): Promise<void> {
    await invoke("section_delete", { id }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    await fetchSections();
  }

  async function sectionNew(
    name: string,
    sectionType: SectionType,
    params: number[],
  ): Promise<void> {
    await invoke("section_new", { name, sectionType, params }).catch(
      (e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      },
    );
    await fetchSections();
  }

  async function sectionUpdate(
    id: number,
    name: string,
    sectionType: SectionType,
    params: number[],
  ): Promise<void> {
    await invoke("section_update", { id, name, sectionType, params }).catch(
      (e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      },
    );
    await fetchSections();
  }

  function select(nodes: number[], members: number[]): void {
    current.value.selected.nodes = [...nodes];
    current.value.selected.members = [...members];
  }

  async function selectedApplyHinges(
    onNodes: boolean,
    onMemberStarts: boolean,
    onMemberEnds: boolean,
  ): Promise<void> {
    const selected = current.value.selected;
    if (selected.nodes.length == 0 && selected.members.length == 0) {
      return;
    }

    const result = await invoke("selected_apply_hinges", {
      nodeIds: selected.nodes,
      memberIds: selected.members,
      onNodes,
      onMemberStarts,
      onMemberEnds,
    }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });
    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function selectedApplyMatSec(
    material: number | null,
    section: number | null,
  ): Promise<void> {
    const selected = current.value.selected;
    if (selected.members.length == 0) {
      return;
    }

    let result = [0, 0];

    if (material !== null && section !== null) {
      const response = await invoke("selected_apply_material_and_section", {
        ids: selected.members,
        materialId: material,
        sectionId: section,
      }).catch((e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      });
      result = response as [number, number];
    } else if (material !== null) {
      const response = await invoke("selected_apply_material", {
        ids: selected.members,
        materialId: material,
      }).catch((e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      });
      result = response as [number, number];
    } else if (section !== null) {
      const response = await invoke("selected_apply_section", {
        ids: selected.members,
        sectionId: section,
      }).catch((e: string[]) => {
        appAlert(t(e[0], [e[1]]));
      });
      result = response as [number, number];
    }

    historyLength.value.undo = result[0];
    historyLength.value.redo = result[1];
  }

  async function selectedDelete(): Promise<void> {
    const selected = current.value.selected;

    if (selected.members.length == 0 && selected.nodes.length == 0) {
      return;
    }

    const result = await invoke("selected_delete", {
      nodeIds: selected.nodes,
      memberIds: selected.members,
    }).catch((e: string[]) => {
      appAlert(t(e[0], [e[1]]));
    });

    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  return {
    combinationsList,
    loadcasesList,
    materialsList,
    sectionsList,

    canvasProps,
    dialogs,
    current,

    historyLength,

    results,
    resultsExtrema,

    appAlert,
    appCursorSnap,
    appExit,
    appRedo,
    appUndo,

    fileNew,
    fileOpen,
    fileSave,
    fileSaveAs,

    analysisRun,

    combinationApplyFactors,
    combinationDelete,
    combinationNew,
    combinationUpdate,

    fetchCombinations,
    fetchEverything,
    fetchLoadcaseCurrent,
    fetchLoadcases,
    fetchMaterials,
    fetchSections,

    loadcaseChangeCurrent,
    loadcaseDelete,
    loadcaseNew,
    loadcaseUpdate,

    materialDelete,
    materialNew,
    materialUpdate,

    sectionDelete,
    sectionNew,
    sectionUpdate,

    select,
    selectedApplyHinges,
    selectedApplyMatSec,
    selectedDelete,
  };
});
