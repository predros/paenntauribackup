import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow, LogicalPosition } from "@tauri-apps/api/window";
import {
  ICombination,
  ILoadcase,
  IMaterial,
  ISection,
  ClickType,
  SideBarType,
  ResultType,
  ResultsDict,
  ReactionsDict,
  SectionType,
  IMemberResult,
} from "@/types/types";

import useNodeStore from "@/state/nodes";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";

import { IPoint, IRectangle } from "@/helper/math";
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
      height: 0
    } as IRectangle
  });

  const showDialog = ref({
    alert: {
      show: false,
      text: ""
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
      if (results.value.combinations.members == null) return result;
      currentCase = results.value.combinations.members[currentResult.id];
    } else {
      if (results.value.loadcases.members == null) return result;
      currentCase = results.value.loadcases.members[currentResult.id];
    }

    currentCase.forEach((member) => {
      const moment_first = member.moment.at(0);
      const moment_last = member.moment.at(-1);
      if (moment_first != undefined)
        result.moment = Math.max(result.moment, Math.abs(moment_first));
      if (moment_last != undefined)
        result.moment = Math.max(result.moment, Math.abs(moment_last));
      result.moment = Math.max(
        result.moment,
        Math.abs(member.max_moment[1]),
        Math.abs(member.min_moment[1]),
      );

      const shear_first = member.shear.at(0);
      const shear_last = member.shear.at(-1);
      if (shear_first != undefined)
        result.shear = Math.max(result.shear, Math.abs(shear_first));
      if (shear_last != undefined)
        result.shear = Math.max(result.shear, Math.abs(shear_last));
      result.shear = Math.max(result.shear, Math.abs(member.vert_shear[1]));

      const normal_first = member.normal.at(0);
      const normal_last = member.normal.at(-1);
      if (normal_first != undefined)
        result.normal = Math.max(result.normal, Math.abs(normal_first));
      if (normal_last != undefined)
        result.normal = Math.max(result.normal, Math.abs(normal_last));
      result.normal = Math.max(result.normal, Math.abs(member.vert_normal[1]));
    });

    result.moment = result.moment != 0 ? result.moment : 1;
    result.shear = result.shear != 0 ? result.shear : 1;
    result.normal = result.normal != 0 ? result.normal : 1;

    return result;
  });

  async function newFile(): Promise<void> {
    const fileChanged = (await invoke("unsaved_changes")) as boolean;

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

    const result = await invoke("new_file").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );

    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await fetchEverything();
    current.value.sideBarType = SideBarType.Select;
    current.value.clickType = ClickType.Select;
  }

  async function openFile(): Promise<void> {
    const fileChanged = (await invoke("unsaved_changes")) as boolean;

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
      title: t("dialogs.native.openFileTitle"),
      multiple: false,
      filters: [
        {
          name: t("dialogs.native.fileType"),
          extensions: ["pnn"],
        },
      ],
    });

    if (selectedFile == null || Array.isArray(selectedFile)) return;

    const result = await invoke("open_file", { path: selectedFile }).catch(
      (e: string[]) => showAlert(t(e[0], [e[1]])),
    );

    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await fetchEverything();
    current.value.sideBarType = SideBarType.Select;
    current.value.clickType = ClickType.Select;
  }

  async function saveFile(): Promise<void> {
    const currentFile = (await invoke("get_current_file")) as string;

    let savePath: string | null;
    if (currentFile.trim() == "") {
      savePath = await save({
        title: t("dialogs.native.saveFileTitle"),
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
    if (savePath == null) return;

    await invoke("save_file", { path: savePath }).catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
  }

  async function saveFileAs(): Promise<void> {
    const savePath = await save({
      title: t("dialogs.native.saveFileTitle"),
      filters: [
        {
          name: t("dialogs.native.fileType"),
          extensions: ["pnn"],
        },
      ],
    });
    if (savePath == null) return;

    await invoke("save_file", { path: savePath }).catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
  }

  async function exitApp(): Promise<void> {
    const fileChanged = (await invoke("unsaved_changes")) as boolean;
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

  function showAlert(text: string): void {
    showDialog.value.alert.text = text;
    showDialog.value.alert.show = true;
  }

  function select(nodes: number[], members: number[]): void {
    current.value.selected.nodes = [...nodes];
    current.value.selected.members = [...members];
  }

  function snapCursorTo(windowPos: IPoint): void {
    appWindow
      .setCursorPosition(new LogicalPosition(windowPos.x, windowPos.y))
      .catch((e: string[]) => showAlert(t(e[0], [e[1]])));
  }

  async function runAnalysisLinear(): Promise<void> {
    showDialog.value.runningAnalysis = true;
    try {
      const response = await invoke("run_analysis_linear").catch((e) =>
        showAlert(e),
      );
      const result = response as [ResultsDict, ReactionsDict, ResultsDict, ReactionsDict];

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
      showAlert(t(e[0], e[1]));
    } finally {
      showDialog.value.runningAnalysis = false;
    }
  }

  async function fetchEverything(): Promise<void> {
    await settings.fetchSettings();

    await fetchMaterials();
    await fetchSections();
    await nodes.fetchNodes();
    await members.fetchMembers();

    await fetchLoadcases();
    await fetchCombinations();
    await fetchCurrentLoadcaseId();
  }

  async function fetchCurrentLoadcaseId(): Promise<void> {
    const id = await invoke("get_loadcase_current").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    current.value.loadcase = id as number;
  }

  async function fetchLoadcases(): Promise<void> {
    const result = await invoke("get_loadcase_dtos").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    loadcasesList.value = result as ILoadcase[];
  }

  async function fetchCombinations() : Promise<void> {
    const result = await invoke("get_combination_dtos").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    combinationsList.value = result as ICombination[];
  }

  async function fetchMaterials(): Promise<void> {
    const result = await invoke("get_material_dtos").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    materialsList.value = result as IMaterial[];
  }

  async function fetchSections(): Promise<void> {
    const result = await invoke("get_section_dtos").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    sectionsList.value = result as ISection[];
  }

  async function changeCurrentLoadcase(new_id: number): Promise<void> {
    await invoke("set_loadcase_current", { id: new_id }).catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function applyHinges(
    onNodes: boolean,
    onMemberStarts: boolean,
    onMemberEnds: boolean,
  ): Promise<void> {
    const selected = current.value.selected;
    if (selected.nodes.length == 0 && selected.members.length == 0)
      return;

    const result = await invoke("apply_hinges", {
      nodeIds: selected.nodes,
      memberIds: selected.members,
      onNodes,
      onMemberStarts,
      onMemberEnds,
    }).catch((e: string[]) => showAlert(t(e[0], [e[1]])));
    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function applyMatSec(
    material: number | null,
    section: number | null,
  ): Promise<void> {
    const selected = current.value.selected;
    if (selected.members.length == 0) return;

    let result = [0, 0];

    if (material != null && section != null) {
      const response = await invoke("apply_material_and_section", {
        ids: selected.members,
        materialId: material,
        sectionId: section,
      }).catch((e: string[]) => showAlert(t(e[0], [e[1]])));
      result = response as [number, number];
    } else if (material != null) {
      const response = await invoke("apply_material", {
        ids: selected.members,
        materialId: material,
      }).catch((e: string[]) => showAlert(t(e[0], [e[1]])));
      result = response as [number, number];
    } else if (section != null) {
      const response = await invoke("apply_section", {
        ids: selected.members,
        sectionId: section,
      }).catch((e: string[]) => showAlert(t(e[0], [e[1]])));
      result = response as [number, number];
    }

    historyLength.value.undo = result[0];
    historyLength.value.redo = result[1];
  }

  async function newMaterial(
    name: string,
    elasticity: number,
    thermal: number,
  ): Promise<void> {
    await invoke("new_material", { name, elasticity, thermal }).catch(
      (e: string[]) => showAlert(t(e[0], [e[1]])),
    );
    await fetchMaterials();
  }

  async function updateMaterial(
    id: number,
    name: string,
    elasticity: number,
    thermal: number,
  ): Promise<void> {
    await invoke("update_material", { id, name, elasticity, thermal }).catch(
      (e: string[]) => showAlert(t(e[0], [e[1]])),
    );
    await fetchMaterials();
  }

  async function deleteMaterial(id: number): Promise<void> {
    await invoke("delete_material", { id }).catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    await fetchMaterials();
  }

  async function newSection(
    name: string,
    sectionType: SectionType,
    params: number[],
  ): Promise<void> {
    await invoke("new_section", { name, sectionType, params }).catch(
      (e: string[]) => showAlert(t(e[0], [e[1]])),
    );
    await fetchSections();
  }

  async function updateSection(
    id: number,
    name: string,
    sectionType: SectionType,
    params: number[],
  ): Promise<void> {
    await invoke("update_section", { id, name, sectionType, params }).catch(
      (e: string[]) => showAlert(t(e[0], [e[1]])),
    );
    await fetchSections();
  }

  async function deleteSection(id: number): Promise<void> {
    await invoke("delete_section", { id }).catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    await fetchSections();
  }

  async function undo(): Promise<void> {
    const result = await invoke("undo").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function redo(): Promise<void> {
    const result = await invoke("redo").catch((e: string[]) =>
      showAlert(t(e[0], [e[1]])),
    );
    const [undoLen, redoLen] = result as [number, number];
    historyLength.value.undo = undoLen;
    historyLength.value.redo = redoLen;

    await nodes.fetchNodes();
    await members.fetchMembers();
  }

  async function deleteSelected(): Promise<void> {
    const selected = current.value.selected;

    if (selected.members.length == 0 && selected.nodes.length == 0)
      return;

    const result = await invoke("delete_selected", {
      nodeIds: selected.nodes,
      memberIds: selected.members,
    }).catch((e: string[]) => showAlert(t(e[0], [e[1]])));

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
    showDialog,
    current,

    historyLength,

    results,
    resultsExtrema,

    exitApp,
    newFile,
    openFile,
    saveFile,
    saveFileAs,

    runAnalysisLinear,

    showAlert,
    select,
    snapCursorTo,

    fetchEverything,
    fetchCurrentLoadcaseId,
    fetchLoadcases,
    fetchCombinations,
    fetchMaterials,
    fetchSections,

    changeCurrentLoadcase,
    applyHinges,
    applyMatSec,

    newMaterial,
    updateMaterial,
    deleteMaterial,

    newSection,
    updateSection,
    deleteSection,

    undo,
    redo,
    deleteSelected,
  };
});
