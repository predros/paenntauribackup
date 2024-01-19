import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { IMember } from "@/types/types";
import { invoke } from "@tauri-apps/api/tauri";
import useGlobalStore from "@/state/global";
import useNodeStore from "@/state/nodes";
import useSettings from "@/state/settings";
import { useI18n } from "vue-i18n";

export default defineStore("members", () => {
  const { t } = useI18n();

  const store = useGlobalStore();
  const nodes = useNodeStore();
  const settings = useSettings();

  const membersList = ref<IMember[]>([]);

  const loadsExtrema = computed(() => {
    let max = 0;
    let min = Number.POSITIVE_INFINITY;

    membersList.value.forEach((member) => {
      if (member.qy0 != 0) {
        max = Math.max(max, Math.abs(member.qy0));
        min = Math.min(min, Math.abs(member.qy0));
      }

      if (member.qy1 != 0) {
        max = Math.max(max, Math.abs(member.qy1));
        min = Math.min(min, Math.abs(member.qy1));
      }

      if (member.is_global) {
        if (member.qx0 != 0) {
          max = Math.max(max, Math.abs(member.qx0));
          min = Math.min(min, Math.abs(member.qx0));
        }
        if (member.qx1 != 0) {
          max = Math.max(max, Math.abs(member.qx1));
          min = Math.min(min, Math.abs(member.qx1));
        }
      }
    });
    return { max, min };
  });

  async function fetchMembers(): Promise<void> {
    const result = await invoke("get_member_dtos").catch((e: string[]) =>
      store.showAlert(t(e[0], [e[1]])),
    );
    membersList.value = result as IMember[];
  }

  async function newMember(
    x0: number,
    y0: number,
    x1: number,
    y1: number,
    materialId: number,
    sectionId: number,
    convert = true,
  ): Promise<void> {
    let realX0: number;
    let realY0: number;
    let realX1: number;
    let realY1: number;

    if (convert) {
      realX0 = settings.lengthToCm(x0);
      realY0 = settings.lengthToCm(y0);
      realX1 = settings.lengthToCm(x1);
      realY1 = settings.lengthToCm(y1);
    } else {
      realX0 = x0;
      realY0 = y0;
      realX1 = x1;
      realY1 = y1;
    }

    const result = await invoke("new_member", {
      x0: realX0,
      y0: realY0,
      x1: realX1,
      y1: realY1,
      materialId,
      sectionId,
    }).catch((e: string[]) => store.showAlert(t(e[0], [e[1]])));

    const [undoLen, redoLen] = result as [number, number];
    store.historyLength.undo = undoLen;
    store.historyLength.redo = redoLen;

    await nodes.fetchNodes();
    await fetchMembers();
  }

  async function applyLoads(
    qx0: number,
    qy0: number,
    qx1: number,
    qy1: number,
    isGlobal: boolean,
  ): Promise<void> {
    const ids = store.current.selected.members;

    if (ids.length == 0) return;

    const result = await invoke("apply_member_loads", {
      ids,
      qx0,
      qy0,
      qx1,
      qy1,
      isGlobal,
    }).catch((e: string[]) => store.showAlert(t(e[0], [e[1]])));

    const [undoLen, redoLen] = result as [number, number];
    store.historyLength.undo = undoLen;
    store.historyLength.redo = redoLen;

    await fetchMembers();
  }

  async function applyTemperatures(tSup: number, tInf: number): Promise<void> {
    const ids = store.current.selected.members;

    if (ids.length == 0) return;

    const result = await invoke("apply_member_temperatures", {
      ids,
      tSup,
      tInf,
    }).catch((e: string[]) => store.showAlert(t(e[0], [e[1]])));

    const [undoLen, redoLen] = result as [number, number];
    store.historyLength.undo = undoLen;
    store.historyLength.redo = redoLen;

    await fetchMembers();
  }

  return {
    membersList,
    loadsExtrema,
    fetchMembers,
    newMember,
    applyLoads,
    applyTemperatures,
  };
});
