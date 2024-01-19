<template>
  <v-app class="app">
    <v-app-bar density="compact" :height="50" elevation="2">
      <MenuComponent />
    </v-app-bar>

    <div class="home-view" tabindex="0" @keydown="onKeyDown">
      <v-navigation-drawer
        v-model="sideBarShow"
        location="right"
        absolute
        :scrim="false"
        :width="350"
        temporary
      >
        <SideBarBase />
      </v-navigation-drawer>

      <div ref="upperRow" class="upper-row">
        <ToolbarTop />
      </div>

      <div class="middle-row">
        <div class="middle-left-cell">
          <ToolbarSide />
        </div>
        <div ref="divCanvas" class="middle-right-cell">
          <MainCanvas
            ref="cnv"
            :width="canvasWidth"
            :height="canvasHeight"
            @mouse-down="onCanvasMouseDown"
            @mouse-up="onCanvasMouseUp"
            @member-clicked="onMemberClicked"
            @node-clicked="onNodeClicked"
          />
        </div>
      </div>

      <div class="bottom-row">
        <v-btn-toggle
          v-model="store.canvasProps.showGrid"
          variant="outlined"
          color="primary"
          density="comfortable"
          rounded="0"
        >
          <v-btn icon :value="true">
            <v-icon v-if="store.canvasProps.showGrid"> mdi-grid </v-icon>
            <v-icon v-else> mdi-grid-off </v-icon>
            <v-tooltip activator="parent" location="top">
              {{ t("app.showGrid") }}
            </v-tooltip>
          </v-btn>
        </v-btn-toggle>
        <v-btn-toggle
          v-model="store.canvasProps.gridSnap"
          variant="outlined"
          color="primary"
          density="comfortable"
          rounded="0"
        >
          <v-btn icon :value="true" :disabled="!store.canvasProps.showGrid">
            <v-icon> mdi-dots-grid </v-icon>
            <v-tooltip activator="parent" location="top">
              {{ t("app.snapToGrid") }}
            </v-tooltip>
          </v-btn>
        </v-btn-toggle>

        <v-btn
          icon
          rounded="0"
          density="comfortable"
          :disabled="store.canvasProps.scale >= 5 / 1.1"
          @click="onZoomIn"
        >
          <v-icon> mdi-magnify-plus </v-icon>
          <v-tooltip activator="parent" location="top">
            {{ t("app.zoomIn") }}
          </v-tooltip>
        </v-btn>

        <v-btn
          icon
          rounded="0"
          density="comfortable"
          :disabled="store.canvasProps.scale <= 0.33"
          @click="onZoomOut"
        >
          <v-icon> mdi-magnify-minus </v-icon>
          <v-tooltip activator="parent" location="top">
            {{ t("app.zoomOut") }}
          </v-tooltip>
        </v-btn>
        <div class="d-flex pl-3 h-100 align-center text-body-2">
          {{ mousePositionText }}
        </div>
      </div>

      <v-dialog
        v-model="settings.showSettingsDialog"
        persistent
        max-width="950"
      >
        <DialogSettings />
      </v-dialog>

      <v-dialog
        v-model="store.showDialog.loadcases"
        persistent
        max-width="500"
      >
        <DialogLoadcases />
      </v-dialog>

      <v-dialog v-model="store.showDialog.runningAnalysis" persistent width="250">
        <v-card>
          <template #loader>
            <v-progress-linear
              :active="true"
              color="primary"
              height="4"
              indeterminate
            />
          </template>
          <v-card-text> Executando análise... </v-card-text>
        </v-card>
      </v-dialog>
    </div>



    <v-snackbar v-model="store.showDialog.alert.show" :timeout="4000">
      {{ store.showDialog.alert.text }}
    </v-snackbar>
  </v-app>
</template>

<script setup lang="ts">
//#region Imports
import { appWindow } from "@tauri-apps/api/window";
import { ref, onMounted, computed } from "vue";
import { VAppBar } from "vuetify/components";
import { invoke } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";

import { ClickType, SideBarType } from "@/types/types";

import {
  doesLineIntersectRect,
  isPointInRect,
  isLineInRect,
  projectOntoVector,
  IRectangle,
  IPoint,
} from "@/helper/math";

import useGlobalStore from "@/state/global";
import useNodeStore from "@/state/nodes";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";

import MenuComponent from "@/components/toolbars/MenuComponent.vue";
import MainCanvas from "@/components/canvas/MainCanvas.vue";
import ToolbarTop from "@/components/toolbars/ToolbarTop.vue";
import ToolbarSide from "@/components/toolbars/ToolbarSide.vue";
import SideBarBase from "./components/sidebars/SideBarBase.vue";
import DialogSettings from "./components/dialogs/DialogSettings.vue";
import { UnitType } from "./types/units";
import DialogLoadcases from "./components/dialogs/DialogLoadcases.vue";
//#endregion

//#region Store definitions
const { t } = useI18n();
const store = useGlobalStore();
const nodes = useNodeStore();
const members = useMemberStore();
const settings = useSettings();
//#endregion

//#region Reactive properties
const upperRow = ref<HTMLElement>();
const divCanvas = ref<HTMLElement>();
const cnv = ref<InstanceType<typeof MainCanvas>>();

const canvasWidth = ref<number>(0);
const canvasHeight = ref<number>(0);
const mouseAnchor = ref<IPoint>({ x: 0, y: 0 });
//#endregion

//#region onMounted
onMounted(async () => {
  await onResize();
  window.addEventListener("resize", onResize);

  await store.newFile();

  await store.fetchEverything();

  invoke("close_splashscreen");
});
//#endregion

//#region Computed properties
const sideBarShow = computed<boolean>(
  () => store.current.sideBarType != SideBarType.Select,
);

const mousePositionText = computed(() => {
  if (store.canvasProps.mousePosition == null) return "";

  const textX = settings.formatUnit(
    store.canvasProps.mousePosition.x,
    UnitType.Length,
    true,
  );
  const textY = settings.formatUnit(
    -store.canvasProps.mousePosition.y,
    UnitType.Length,
    true,
  );

  return `X: ${textX}; Y: ${textY}`;
});
//#endregion

//#region Functions
function snapToGrid(x: number, y: number) {
  if (!cnv.value) return null;
  if (!divCanvas.value) return null;

  const stage = cnv.value.getStage();
  if (!stage) return null;

  const canvasPosition = divCanvas.value.getBoundingClientRect();

  const snapCoords: IPoint = {
    x: Math.round(x / settings.gridSpacing.x) * settings.gridSpacing.x,
    y: Math.round(y / settings.gridSpacing.y) * settings.gridSpacing.y,
  };

  const windowCoords: IPoint = {
    x: snapCoords.x * store.canvasProps.scale + stage.x() + canvasPosition.x,
    y: snapCoords.y * store.canvasProps.scale + stage.y() + canvasPosition.y,
  };

  return { coords: snapCoords, window: windowCoords };
}
//#endregion

//#region Event handlers
async function onResize(): Promise<void> {
  if (!upperRow.value) return;

  const size = await appWindow.innerSize();
  const viewHeight = size.height;
  const viewWidth = size.width;
  const upperHeight = upperRow.value.clientHeight;

  canvasWidth.value = viewWidth - 110;
  canvasHeight.value = viewHeight - upperHeight - 100;
}

async function onKeyDown(event: KeyboardEvent): Promise<void> {
  if (event.ctrlKey && !event.shiftKey && !event.altKey) {
    switch (event.key.toLowerCase()) {
      case "n":
        await store.newFile();
        break;
      case "o":
        await store.openFile();
        break;
      case "s":
        await store.saveFile();
        break;
      case "w":
        await store.exitApp();
        break;
      case "y":
        if (store.current.sideBarType != SideBarType.Result) await store.redo();
        break;
      case "z":
        if (store.current.sideBarType != SideBarType.Result) await store.undo();
        break;
    }
  } else if (event.ctrlKey && event.shiftKey && !event.altKey) {
    switch (event.key.toLowerCase()) {
      case "s":
        await store.saveFileAs();
        break;
    }
  } else if (!event.ctrlKey && !event.shiftKey && !event.altKey) {
    switch (event.key.toLowerCase()) {
      case "delete":
        await store.deleteSelected();
        break;
    }
  }
}

async function onCanvasMouseDown(x: number, y: number): Promise<void> {
  if (!cnv.value) return;

  switch (store.current.clickType) {
    case ClickType.Select:
      mouseAnchor.value = { x, y };
      break;
    case ClickType.NewNode:
      if (store.canvasProps.gridSnap) {
        const snap = snapToGrid(x, y);
        if (snap == null) return;
        await nodes.newNode(snap.coords.x, -snap.coords.y, false);
        store.snapCursorTo(snap.window);
      } else {
        await nodes.newNode(x, -y, false);
      }
      break;
    default:
      break;
  }
}

async function onCanvasMouseUp(
  x: number,
  y: number,
  xAnchor: number | undefined,
  yAnchor: number | undefined,
): Promise<void> {
  if (!cnv.value) return;

  switch (store.current.clickType) {
    case ClickType.Select:
      if (xAnchor == undefined || yAnchor == undefined) store.select([], []);
      else {
        const width = x - xAnchor;
        const height = y - yAnchor;
        const rect: IRectangle = { x: xAnchor, y: yAnchor, width, height };

        const selectedNodes: number[] = [];
        const selectedMembers: number[] = [];

        nodes.nodesList.forEach((node) => {
          if (isPointInRect({ x: node.x, y: -node.y }, rect))
            selectedNodes.push(node.id);
        });

        if (width > 0) {
          members.membersList.forEach((member) => {
            if (
              isLineInRect(
                {
                  start: { x: member.x0, y: -member.y0 },
                  end: { x: member.x1, y: -member.y1 },
                },
                rect,
              )
            )
              selectedMembers.push(member.id);
          });
        } else {
          members.membersList.forEach((member) => {
            if (
              doesLineIntersectRect(
                {
                  start: { x: member.x0, y: -member.y0 },
                  end: { x: member.x1, y: -member.y1 },
                },
                rect,
              )
            )
              selectedMembers.push(member.id);
          });
        }
        store.select(selectedNodes, selectedMembers);
      }
      break;
    case ClickType.NewMemberStart:
      if (xAnchor == undefined || yAnchor == undefined) return;
      if (store.canvasProps.gridSnap) {
        const snap = snapToGrid(xAnchor, yAnchor);
        if (snap == null) return;

        store.canvasProps.newMemberAnchor = snap.coords;
        store.snapCursorTo(snap.window);
        store.current.clickType = ClickType.NewMemberEnd;
      } else {
        store.canvasProps.newMemberAnchor = { x: xAnchor, y: yAnchor };
        store.current.clickType = ClickType.NewMemberEnd;
      }
      break;
    case ClickType.NewMemberEnd:
      if (store.canvasProps.newMemberAnchor == null) return;

      if (store.current.material == null) {
        store.showAlert(t("alerts.noMaterialSelected"));
        return;
      } else if (store.current.section == null) {
        store.showAlert(t("alerts.noSectionSelected"));
        return;
      }

      if (store.canvasProps.gridSnap) {
        const snap = snapToGrid(x, y);
        if (snap == null) return;

        await members.newMember(
          store.canvasProps.newMemberAnchor.x,
          -store.canvasProps.newMemberAnchor.y,
          snap.coords.x,
          -snap.coords.y,
          store.current.material,
          store.current.section,
          false,
        );
        store.snapCursorTo(snap.window);
      } else {
        await members.newMember(
          store.canvasProps.newMemberAnchor.x,
          -store.canvasProps.newMemberAnchor.y,
          x,
          -y,
          store.current.material,
          store.current.section,
          false,
        );
      }
      store.current.clickType = ClickType.NewMemberStart;
      store.canvasProps.newMemberAnchor = null;
      break;
    default:
      break;
  }
}

function onZoomIn(): void {
  if (!cnv.value) return;
  cnv.value.zoom(1.1, false);
}

function onZoomOut(): void {
  if (!cnv.value) return;
  cnv.value.zoom(1 / 1.1, false);
}

async function onNodeClicked(id: number): Promise<void> {
  const node = nodes.getNode(id);
  switch (store.current.clickType) {
    case ClickType.NewNode:
      store.showAlert(t("alerts.nodeAlreadyExists", [id]));
      break;
    case ClickType.Select:
      store.select([id], []);
      break;
    case ClickType.NewMemberStart:
      if (node == undefined) return;
      store.canvasProps.newMemberAnchor = { x: node.x, y: -node.y };
      store.current.clickType = ClickType.NewMemberEnd;
      break;
    case ClickType.NewMemberEnd:
      if (node == undefined) return;
      if (store.current.material == null) {
        store.showAlert(t("alerts.noMaterialSelected"));
        return;
      } else if (store.current.section == null) {
        store.showAlert(t("alerts.noSectionSelected"));
        return;
      }

      if (store.canvasProps.newMemberAnchor == null) return;

      await members.newMember(
        store.canvasProps.newMemberAnchor.x,
        -store.canvasProps.newMemberAnchor.y,
        node.x,
        node.y,
        store.current.material,
        store.current.section,
        false,
      );
      store.current.clickType = ClickType.NewMemberStart;
      break;
    default:
      break;
  }
}

async function onMemberClicked(
  id: number,
  mouseX: number,
  mouseY: number,
): Promise<void> {
  switch (store.current.clickType) {
    case ClickType.Select:
      store.select([], [id]);
      break;
    case ClickType.NewNode:
      if (store.canvasProps.gridSnap) {
        const snap = snapToGrid(mouseX, mouseY);
        if (snap == null) return;

        await nodes.newNode(snap.coords.x, -snap.coords.y, false);
        store.snapCursorTo(snap.window);
      } else {
        const member = members.membersList.find((x) => x.id == id);
        if (!member) return;

        const memberVector = {
          x: member.x1 - member.x0,
          y: member.y1 - member.y0,
        };
        const mouseVector = {
          x: mouseX - member.x0,
          y: -mouseY - member.y0,
        };
        const projected = projectOntoVector(memberVector, mouseVector);
        if (projected == null) return;

        await nodes.newNode(
          member.x0 + projected.x,
          member.y0 + projected.y,
          false,
        );
      }
      break;
    case ClickType.Result:
      {
        const member = members.membersList.find((x) => x.id == id);
        if (!member) return;

        const memberVector = {
          x: member.x1 - member.x0,
          y: member.y1 - member.y0,
        };
        const mouseVector = {
          x: mouseX - member.x0,
          y: -mouseY - member.y0,
        };
        const projected = projectOntoVector(memberVector, mouseVector);
        if (projected == null) return;

        const angle = (member.angle * Math.PI) / 180;
        const cos = Math.cos(angle);
        const sin = Math.sin(angle);

        const memberCoord = projected.x * cos + projected.y * sin;

        store.results.selected.id = id;
        store.results.selected.position = memberCoord;
      }
      break;
    default:
      break;
  }
}

//#endregion
</script>

<style scoped>
.home-view {
  display: flex;
  flex-flow: column;
  padding-top: 45px;
  margin: 0;
  width: 100vw;
}

.upper-row {
  display: flex;
  flex-flow: column;
  padding-left: 10px;
  padding-right: 30px;
}

.middle-row {
  display: flex;
  flex-flow: row;
}

.bottom-row {
  display: flex;
  flex-flow: row;
  width: 100%;
  min-height: 50px;
  padding-left: 80px;
  padding-top: 5px;
  padding-bottom: 10px;
}

.middle-left-cell {
  padding-left: 10px;
  width: 80px;
  height: 100%;
}

.middle-right-cell {
  border-style: solid;
  border-width: 1px;
  border-color: #d4d4d4;
}
</style>
