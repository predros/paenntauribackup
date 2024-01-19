<template>
  <div
    ref="container"
    style="background: #ffffff; width: fit-content; height: fit-content"
  >
    <v-stage
      ref="cnv"
      :config="cnvConfig"
      @wheel="onWheel"
      @mousedown="onMouseDown"
      @mouseup="onMouseUp"
      @mousemove="onMouseMove"
      @dragmove="onDragMove"
      @mouseenter="onMouseEnter"
      @mouseleave="onMouseLeave"
    >
      <v-layer>
        <v-rect
          v-if="store.current.clickType == ClickType.Select && mouseAnchor != null"
          :config="selectionRectConfig"
        />
        <v-line
          v-if="
            store.current.clickType == ClickType.NewMemberEnd &&
            store.canvasProps.newMemberAnchor != null
          "
          :config="tempMemberConfig"
        />

        <GridComponent v-if="store.canvasProps.showGrid" />

        <MemberComponent
          v-for="member in membersVisible.total"
          :key="member.id"
          :member="member"
          :scale="store.canvasProps.scale"
          :selected="store.current.selected.members.includes(member.id)"
          @clicked="onMemberClicked"
        />

        <MemberResult
          v-for="member in membersVisible.result.member"
          :key="member.id"
          :member="member"
          :scale="store.canvasProps.scale"
          :result-type="store.results.type"
          :result="findMemberResult(member.id)"
          :result-scale="store.results.scale"
          :extrema="store.resultsExtrema"
          :selected="store.results.selected.id == member.id"
          :selected-position="store.results.selected.position || 0"
        />

        <LoadLocalX
          v-for="member in membersVisible.loads.xLocal"
          :key="member.id"
          :member="member"
          :scale="store.canvasProps.scale"
        />

        <LoadLocalY
          v-for="member in membersVisible.loads.yLocal"
          :key="member.id"
          :member="member"
          :scale="store.canvasProps.scale"
          :extrema="members.loadsExtrema"
        />

        <LoadGlobal
          v-for="member in membersVisible.loads.global"
          :key="member.id"
          :member="member"
          :scale="store.canvasProps.scale"
          :extrema="members.loadsExtrema"
        />

        <NodeComponent
          v-for="node in nodesVisible.total"
          :key="node.id"
          :node="node"
          :selected="store.current.selected.nodes.includes(node.id)"
          :scale="store.canvasProps.scale"
          @clicked="onNodeClicked"
        />
        <SupportComponent
          v-for="node in nodesVisible.total"
          :key="node.id"
          :node="node"
          :scale="store.canvasProps.scale"
        />
        <SpringComponent
          v-for="node in nodesVisible.springs.x"
          :key="node.id"
          :node="node"
          :direction="Direction.X"
          :scale="store.canvasProps.scale"
        />
        <SpringComponent
          v-for="node in nodesVisible.springs.y"
          :key="node.id"
          :node="node"
          :direction="Direction.Y"
          :scale="store.canvasProps.scale"
        />
        <SpringComponent
          v-for="node in nodesVisible.springs.z"
          :key="node.id"
          :node="node"
          :direction="Direction.Z"
          :scale="store.canvasProps.scale"
        />

        <NodalReaction
          v-for="node in nodesVisible.result.x"
          :key="node.id"
          :node="node"
          :direction="Direction.X"
          :scale="store.canvasProps.scale"
          :reactions="findNodeReaction(node.id)"
        />
        <NodalReaction
          v-for="node in nodesVisible.result.y"
          :key="node.id"
          :node="node"
          :direction="Direction.Y"
          :scale="store.canvasProps.scale"
          :reactions="findNodeReaction(node.id)"
        />
        <MomentReaction
          v-for="node in nodesVisible.result.z"
          :key="node.id"
          :node="node"
          :scale="store.canvasProps.scale"
          :reactions="findNodeReaction(node.id)"
        />

        <NodeDisplacement
          v-for="node in nodesVisible.displacements.x"
          :key="node.id"
          :node="node"
          :direction="Direction.X"
          :scale="store.canvasProps.scale"
        />
        <NodeDisplacement
          v-for="node in nodesVisible.displacements.y"
          :key="node.id"
          :node="node"
          :direction="Direction.Y"
          :scale="store.canvasProps.scale"
        />
        <NodeRotation
          v-for="node in nodesVisible.displacements.z"
          :key="node.id"
          :node="node"
          :scale="store.canvasProps.scale"
        />

        <NodalForce
          v-for="node in nodesVisible.forces.x"
          :key="node.id"
          :node="node"
          :direction="Direction.X"
          :scale="store.canvasProps.scale"
          :extrema="nodes.forcesExtrema"
        />
        <NodalForce
          v-for="node in nodesVisible.forces.y"
          :key="node.id"
          :node="node"
          :direction="Direction.Y"
          :scale="store.canvasProps.scale"
          :extrema="nodes.forcesExtrema"
        />
        <NodalMoment
          v-for="node in nodesVisible.forces.z"
          :key="node.id"
          :node="node"
          :scale="store.canvasProps.scale"
        />
      </v-layer>
    </v-stage>
  </div>
</template>

<script setup lang="ts">
//#region Imports
import { ref, defineProps, computed, defineEmits, watch } from "vue";
import { Stage } from "konva/lib/Stage";
import {
  INode,
  INodeReaction,
  IMember,
  ClickType,
  IMemberResult,
  KonvaMouseEvent,
  KonvaWheelEvent,
  Direction,
} from "@/types/types";
import {
  IRectangle,
  IPoint,
  isPointInRect,
  doesLineIntersectRect,
  floatEq,
} from "@/helper/math";

import NodeComponent from "@/components/canvas/nodes/NodeComponent.vue";
import MemberComponent from "@/components/canvas/members/MemberComponent.vue";
import SupportComponent from "@/components/canvas/nodes/SupportComponent.vue";
import SpringComponent from "@/components/canvas/nodes/SpringComponent.vue";
import NodalForce from "@/components/canvas/nodes/NodalForce.vue";
import NodeDisplacement from "@/components/canvas/nodes/NodeDisplacement.vue";
import NodeRotation from "@/components/canvas/nodes/NodeRotation.vue";
import GridComponent from "@/components/canvas/GridComponent.vue";
import NodalMoment from "@/components/canvas/nodes/NodalMoment.vue";
import LoadLocalX from "@/components/canvas/members/LoadLocalX.vue";
import LoadLocalY from "@/components/canvas/members/LoadLocalY.vue";
import LoadGlobal from "@/components/canvas/members/LoadGlobal.vue";

import useGlobalStore from "@/state/global";
import useNodeStore from "@/state/nodes";
import useMemberStore from "@/state/members";
import MemberResult from "./members/MemberResult.vue";
import NodalReaction from "./nodes/NodalReaction.vue";
import MomentReaction from "./nodes/MomentReaction.vue";
//#endregion

//#region Type definitions
interface KonvaStage {
  getStage(): Stage;
}
//#endregion

//#region Store definitions
const store = useGlobalStore();
const nodes = useNodeStore();
const members = useMemberStore();
//#endregion

//#region Props, emits and exposes
const emit = defineEmits([
  "mouseDown",
  "mouseUp",
  "nodeClicked",
  "memberClicked",
]);
defineExpose({ updateViewPort, zoom, getCursorPosition, getStage });

const props = defineProps({
  width: {
    type: Number,
    default: 0,
  },
  height: {
    type: Number,
    default: 0,
  },
});
//#endregion

//#region Reactive properties
const cnv = ref<KonvaStage>();

const zoomToggle = ref<boolean>(true);
const mouseAnchor = ref<IPoint | null>(null);
//#endregion

//#region Computed properties
const nodesVisible = computed(() => {
  const ret = {
    total: [] as INode[],
    springs: {
      x: [] as INode[],
      y: [] as INode[],
      z: [] as INode[],
    },
    displacements: {
      x: [] as INode[],
      y: [] as INode[],
      z: [] as INode[],
    },
    forces: {
      x: [] as INode[],
      y: [] as INode[],
      z: [] as INode[],
    },
    result: {
      x: [] as INode[],
      y: [] as INode[],
      z: [] as INode[],
      result: [] as INodeReaction[],
    },
  };

  let currentReactions: INodeReaction[];
  if (store.current.result.isCombination) {
    if (store.results.combinations.reactions != null)
      currentReactions = store.results.combinations.reactions[store.current.result.id];
  } else {
    if (store.results.loadcases.reactions != null)
      currentReactions = store.results.loadcases.reactions[store.current.result.id];
  }

  nodes.nodesList.forEach((node: INode) => {
    if (isPointInRect({ x: node.x, y: -node.y }, store.canvasProps.viewPortBounds)) {
      ret.total.push(node);
      if (!floatEq(node.springs[0], 0)) ret.springs.x.push(node);
      if (!floatEq(node.springs[1], 0)) ret.springs.y.push(node);
      if (!floatEq(node.springs[2], 0)) ret.springs.z.push(node);

      if (store.current.clickType != ClickType.Result) {
        if (!floatEq(node.fx, 0)) ret.forces.x.push(node);
        if (!floatEq(node.fy, 0)) ret.forces.y.push(node);
        if (!floatEq(node.mz, 0)) ret.forces.z.push(node);

        if (!floatEq(node.prescribed_displacements[0], 0))
          ret.displacements.x.push(node);
        if (!floatEq(node.prescribed_displacements[1], 0))
          ret.displacements.y.push(node);
        if (!floatEq(node.prescribed_displacements[2], 0))
          ret.displacements.z.push(node);
      } else if (currentReactions != null) {
        const nodeReaction = currentReactions.find((x) => x.id == node.id);
        if (nodeReaction != undefined) {
          ret.result.result.push(nodeReaction);
          if (!floatEq(nodeReaction.rx, 0)) ret.result.x.push(node);
          if (!floatEq(nodeReaction.ry, 0)) ret.result.y.push(node);
          if (!floatEq(nodeReaction.mz, 0)) ret.result.z.push(node);
        }
      }
    }
  });
  return ret;
});

const membersVisible = computed(() => {
  const ret = {
    total: [] as IMember[],
    loads: {
      xLocal: [] as IMember[],
      yLocal: [] as IMember[],
      global: [] as IMember[],
    },
    result: {
      member: [] as IMember[],
      result: [] as IMemberResult[],
    },
  };

  let currentResults: IMemberResult[];

  if (store.current.result.isCombination) {
    if (store.results.combinations.members != null)
      currentResults = store.results.combinations.members[store.current.result.id];
  } else {
    if (store.results.loadcases.members != null)
      currentResults = store.results.loadcases.members[store.current.result.id];
  }

  members.membersList.forEach((member: IMember) => {
    if (
      doesLineIntersectRect(
        {
          start: { x: member.x0, y: -member.y0 },
          end: { x: member.x1, y: -member.y1 },
        },
        store.canvasProps.viewPortBounds,
      )
    ) {
      ret.total.push(member);

      if (store.current.clickType != ClickType.Result) {
        if (member.is_global) {
          if (
            member.qx0 != 0 ||
            member.qx1 != 0 ||
            member.qy0 != 0 ||
            member.qy1 != 0
          ) {
            ret.loads.global.push(member);
          }
        } else {
          if (member.qx0 != 0 || member.qx1 != 0) {
            ret.loads.xLocal.push(member);
          }

          if (member.qy0 != 0 || member.qy1 != 0) {
            ret.loads.yLocal.push(member);
          }
        }
      } else if (currentResults != null) {
        const memberResult = currentResults.find((x) => x.id == member.id);
        if (memberResult != undefined) {
          ret.result.member.push(member);
          ret.result.result.push(memberResult);
        }
      }
    }
  });
  return ret;
});

const cnvConfig = computed<IRectangle>(() => ({
  x: 300,
  y: 300,
  width: props.width,
  height: props.height,
}));

const selectionRectConfig = computed(() => {
  if (
    store.current.clickType != ClickType.Select ||
    mouseAnchor.value == null ||
    store.canvasProps.mousePosition == null
  )
    return {};

  const width = store.canvasProps.mousePosition.x - mouseAnchor.value.x;
  const height = store.canvasProps.mousePosition.y - mouseAnchor.value.y;

  return {
    x: mouseAnchor.value.x,
    y: mouseAnchor.value.y,
    width,
    height,
    fill: width > 0 ? "#42A5F5" : "#66BB6A",
    stroke: width > 0 ? "#0D47A1" : "#1B5E20",
    opacity: 0.5,
  };
});

const tempMemberConfig = computed(() => {
  if (
    store.current.clickType != ClickType.NewMemberEnd ||
    store.canvasProps.newMemberAnchor == null ||
    store.canvasProps.mousePosition == null
  )
    return {};

  const width = store.canvasProps.mousePosition.x - store.canvasProps.newMemberAnchor.x;
  const height = store.canvasProps.mousePosition.y - store.canvasProps.newMemberAnchor.y;

  return {
    x: store.canvasProps.newMemberAnchor.x,
    y: store.canvasProps.newMemberAnchor.y,
    points: [0, 0, width, height],
    strokeWidth: 1.75,
    stroke: "#4682b4",
    opacity: 0.65,
  };
});
//#endregion

//#region Functions
function updateViewPort(): void {
  if (!cnv.value) return;

  const widthBuffer = 0.5;
  const heightBuffer = 0.5;

  const stage = cnv.value.getStage();
  const scale = stage.scaleX();
  const width = props.width / scale;
  const height = props.height / scale;

  const topX = -stage.x() / scale;
  const topY = -stage.y() / scale;

  store.canvasProps.viewPortBounds = {
    x: topX - widthBuffer * width,
    y: topY - heightBuffer * height,
    width: width * (1 + 2 * widthBuffer),
    height: height * (1 + 2 * heightBuffer),
  };
}

function getCursorPosition(): { pointer: IPoint; mouseCoords: IPoint } | null {
  if (!cnv.value)
    return {
      pointer: { x: 0, y: 0 },
      mouseCoords: { x: 0, y: 0 },
    };

  const stage = cnv.value.getStage();
  const oldScale = stage.scaleX();
  const pointer = stage.getPointerPosition();

  if (pointer == null) return null;

  return {
    pointer: pointer,
    mouseCoords: {
      x: (pointer.x - stage.x()) / oldScale,
      y: (pointer.y - stage.y()) / oldScale,
    },
  };
}

function getStage(): Stage | undefined {
  return cnv.value?.getStage();
}

function zoom(factor: number, onCursor: boolean): void {
  if (!cnv.value) return;
  const stage = cnv.value.getStage();
  const oldScale = stage.scaleX();
  const newScale = oldScale * factor;
  if (newScale > 5 || newScale < 0.5) return;

  let newPos = { x: 0, y: 0 };
  if (onCursor) {
    const cursorPos = getCursorPosition();
    if (cursorPos == null) return;
    const { pointer, mouseCoords } = cursorPos;

    newPos = {
      x: pointer.x - mouseCoords.x * newScale,
      y: pointer.y - mouseCoords.y * newScale,
    };
  } else {
    newPos = {
      x:
        stage.width() / 2 -
        ((stage.width() / 2 - stage.x()) * newScale) / oldScale,
      y:
        stage.height() / 2 -
        ((stage.height() / 2 - stage.y()) * newScale) / oldScale,
    };
  }

  store.canvasProps.scale = newScale;
  stage.scale({ x: newScale, y: newScale });

  stage.position(newPos);
  updateViewPort();
}

function findMemberResult(id: number): IMemberResult | undefined {
  const m = membersVisible.value.result.result.find((x) => x.id == id);
  return m;
}

function findNodeReaction(id: number): INodeReaction | undefined {
  const n = nodesVisible.value.result.result.find((x) => x.id == id);
  return n;
}
//#endregion

//#region Event handlers
function onWheel(e: KonvaWheelEvent): void {
  if (!cnv.value) return;

  if (!zoomToggle.value) {
    zoomToggle.value = !zoomToggle.value;
    return;
  }

  // Direction to zoom (in or out)
  let direction = e.evt.deltaY < 0 ? 1 : -1;

  // If zooming on trackpad, e.evt.ctrlKey is true;
  // in that case, revert direction
  if (e.evt.ctrlKey) {
    direction = -direction;
  }

  // Set the new canvas scale, and pass it to the store
  const scaleBy = direction > 0 ? 1.1 : 1 / 1.1;

  zoom(scaleBy, true);
  zoomToggle.value = !zoomToggle.value;
}

function onMouseDown(e: KonvaMouseEvent): void {
  e.evt.preventDefault();

  if (!cnv.value) return;

  const cursorPos = getCursorPosition();
  if (cursorPos == null) return;
  const { mouseCoords } = cursorPos;

  if (e.evt.buttons == 1) {
    mouseAnchor.value = { x: mouseCoords.x, y: mouseCoords.y };
    emit("mouseDown", mouseCoords.x, mouseCoords.y);
  } else if (e.evt.buttons == 4) {
    cnv.value.getStage().draggable(true);
  }
}

function onMouseUp(e: KonvaMouseEvent): void {
  e.evt.preventDefault();

  if (!cnv.value) return;
  const cursorPos = getCursorPosition();
  if (cursorPos == null) return;
  const { mouseCoords } = cursorPos;

  cnv.value.getStage().draggable(false);

  if (e.evt.button == 0) {
    emit(
      "mouseUp",
      mouseCoords.x,
      mouseCoords.y,
      mouseAnchor.value?.x,
      mouseAnchor.value?.y,
    );
    mouseAnchor.value = null;
  }
}

function onMouseMove(e: KonvaMouseEvent): void {
  e.evt.preventDefault();
  if (!cnv.value) return;

  const cursorPos = getCursorPosition();
  if (cursorPos == null) return;
  const { mouseCoords } = cursorPos;

  store.canvasProps.mousePosition = mouseCoords;
}

function onDragMove(e: KonvaMouseEvent): void {
  e.evt.preventDefault();
  if (!cnv.value) return;
  updateViewPort();
}

function onMouseEnter(e: KonvaMouseEvent): void {
  e.evt.preventDefault();

  if (e.evt.buttons != 1) mouseAnchor.value = null;
}

function onMouseLeave(): void {
  store.canvasProps.mousePosition = null;
  if (store.current.clickType == ClickType.NewMemberEnd)
    store.current.clickType = ClickType.NewMemberStart;
}

function onNodeClicked(id: number): void {
  emit("nodeClicked", id);
}

function onMemberClicked(id: number): void {
  if (!cnv.value) return;
  const cursorPos = getCursorPosition();
  if (cursorPos == null) return;

  emit("memberClicked", id, cursorPos.mouseCoords.x, cursorPos.mouseCoords.y);
}
//#endregion

//#region Watchers
watch(
  () => props.width,
  () => updateViewPort(),
);
watch(
  () => props.height,
  () => updateViewPort(),
);
//#endregion
</script>
