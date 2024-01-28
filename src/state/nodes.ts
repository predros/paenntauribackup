import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { type INode } from "@/types/types";
import { invoke } from "@tauri-apps/api/tauri";
import useMemberStore from "@/state/members";
import useGlobalStore from "@/state/global";
import useSettings from "@/state/settings";
import { useI18n } from "vue-i18n";
import { UnitType } from "@/types/units";

export default defineStore("nodes", () => {
  const { t } = useI18n();

  const store = useGlobalStore();
  const members = useMemberStore();
  const settings = useSettings();

  const nodesList = ref<INode[]>([]);

  async function fetchNodes(): Promise<void> {
    const result = await invoke("node_get_dtos").catch((e: string[]) => {
      store.appAlert(t(e[0], [e[1]]));
    });

    nodesList.value = result as INode[];
  }

  async function newNode(x: number, y: number, convert = true): Promise<void> {
    let realX: number;
    let realY: number;
    if (convert) {
      realX = settings.lengthToCm(x);
      realY = settings.lengthFromCm(y);
    } else {
      realX = x;
      realY = y;
    }

    const result = await invoke("node_new", { x: realX, y: realY }).catch(
      (e: string[]) => {
        store.appAlert(t(e[0], [e[1]]));
      },
    );

    const [undoLen, redoLen] = result as [number, number];
    store.historyLength.undo = undoLen;
    store.historyLength.redo = redoLen;

    await fetchNodes();
    await members.fetchMembers();
  }

  async function applySupports(
    supports: { x: boolean; y: boolean; z: boolean; angle: number },
    springs: { x: number; y: number; z: number },
    displacements: { x: number; y: number; z: number },
  ): Promise<void> {
    const ids = store.current.selected.nodes;

    if (ids.length == 0) {
      return;
    }

    const fullRotation =
      settings.getUnitName(UnitType.Angle) == "rad" ? 2 * Math.PI : 360;

    let angleCorrected = supports.angle % fullRotation;
    if (angleCorrected < 0) {
      angleCorrected += fullRotation;
    }

    const result = await invoke("selected_apply_supports", {
      ids,
      supports: [supports.x, supports.y, supports.z, angleCorrected],
      springs: [springs.x, springs.y, springs.z],
      displacements: [displacements.x, displacements.y, displacements.z],
    }).catch((e: string[]) => {
      store.appAlert(t(e[0], [e[1]]));
    });

    const [undoLen, redoLen] = result as [number, number];
    store.historyLength.undo = undoLen;
    store.historyLength.redo = redoLen;

    await fetchNodes();
  }

  async function applyNodalForces(
    fx: number,
    fy: number,
    mz: number,
    angle: number,
  ): Promise<void> {
    const ids = store.current.selected.nodes;

    if (ids.length == 0) {
      return;
    }

    let angleCorrected = angle % 360;
    if (angleCorrected < 0) {
      angleCorrected += 360;
    }

    const result = await invoke("selected_apply_nodal_forces", {
      ids,
      fx,
      fy,
      mz,
      angle: angleCorrected,
    }).catch((e: string[]) => {
      store.appAlert(t(e[0], [e[1]]));
    });

    const [undoLen, redoLen] = result as [number, number];
    store.historyLength.undo = undoLen;
    store.historyLength.redo = redoLen;

    await fetchNodes();
  }

  function getNode(id: number): INode | undefined {
    const result = nodesList.value.find((x) => x.id == id);
    return result;
  }

  const forcesExtrema = computed(() => {
    let max = 0;
    let min = Number.POSITIVE_INFINITY;

    nodesList.value.forEach((node) => {
      if (node.fx != 0) {
        max = Math.max(max, Math.abs(node.fx));
        min = Math.min(min, Math.abs(node.fx));
      }

      if (node.fy != 0) {
        max = Math.max(max, Math.abs(node.fy));
        min = Math.min(min, Math.abs(node.fy));
      }
    });

    return { max, min };
  });

  return {
    nodesList,
    forcesExtrema,

    fetchNodes,
    newNode,
    applySupports,
    applyNodalForces,
    getNode,
  };
});
