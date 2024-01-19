<template>
  <v-rect
    v-if="!props.node.hinged"
    :config="shapeConfig"
    @mousedown="onMouseDown"
    @mouseup="onMouseUp"
  />

  <v-circle
    v-else
    :config="shapeConfig"
    @mousedown="onMouseDown"
    @mouseup="onMouseUp"
  />
</template>

<script setup lang="ts">
import { defineProps, defineEmits, computed } from "vue";
import { INode, KonvaMouseEvent } from "@/types/types";

const props = defineProps({
  node: {
    type: Object as () => INode,
    default: () => ({
      id: 0,
      x: 0,
      y: 0,
      hinged: false,
      supports: [false, false, false],
      support_angle: 0,
      springs: [0, 0, 0],
      prescribed_displacement: [0, 0, 0],
      fx: 0,
      fy: 0,
      mz: 0,
      force_angle: 0,
    }),
  },
  scale: {
    type: Number,
    default: 1.0,
  },
  selected: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["clicked"]);

const shapeConfig = computed(() => {
  if (props.node.hinged) {
    const result = {
      x: props.node.x,
      y: -props.node.y,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
      hitStrokeWidth: 16,
      radius: 6,
      fill: "white",
      stroke: props.selected ? "#B71C1C" : "black",
      strokeWidth: 2,
    };
    return result;
  } else {
    const result = {
      x: props.node.x,
      y: -props.node.y,
      offsetX: 6,
      offsetY: 6,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
      hitStrokeWidth: 16,
      width: 12,
      height: 12,
      fill: props.selected ? "#B71C1C" : "black",
    };
    return result;
  }
});

function onMouseUp(e: KonvaMouseEvent): void {
  e.cancelBubble = true;
  emit("clicked", props.node.id);
}

function onMouseDown(e: KonvaMouseEvent): void {
  e.cancelBubble = true;
}
</script>
