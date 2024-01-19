<template>
  <v-group :config="shapeConfig.group">
    <v-line :config="shapeConfig.line.axis" />
    <v-line :config="shapeConfig.line.rotated" />
    <v-arrow :config="shapeConfig.arrow" />
    <v-text :config="shapeConfig.text" />
  </v-group>
</template>

<script setup lang="ts">
import { computed, defineProps } from "vue";
import { INode } from "@/types/types";
import { UnitType } from "@/types/units";
import useSettings from "@/state/settings";

const settings = useSettings();

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
});

const text = computed<string>(() =>
  settings.formatUnit(
    Math.abs(props.node.prescribed_displacements[2]),
    UnitType.Rotation,
  ),
);

const shapeConfig = computed(() => {
  const radius = 20;
  const diag = radius * Math.SQRT1_2;

  return {
    arrow: {
      stroke: "#004D40",
      fill: "#004D40",
      strokeWidth: 3,
      points: [-radius, 0, -diag, diag, 0, radius, diag, diag],
      tension: 0.4,
      listening: false,
    },
    line: {
      axis: {
        stroke: "#004D40",
        strokeWidth: 1,
        points: [0, 0, -radius, 0],
      },
      rotated: {
        stroke: "#004D40",
        strokeWidth: 1,
        points: [0, 0, -diag, diag],
      },
    },
    text: {
      x: props.node.prescribed_displacements[2] > 0 ? 12 : 45,
      y: 25,
      listening: false,
      fontFamily: "Roboto",
      text: text.value,
      fill: "#004D40",
      scaleX: props.node.prescribed_displacements[2] > 0 ? 1 : -1,
    },
    group: {
      listening: false,
      x: props.node.x,
      y: -props.node.y,
      scaleY: 1 / props.scale,
      scaleX:
        props.node.prescribed_displacements[2] > 0
          ? 1 / props.scale
          : -1 / props.scale,
    },
  };
});
</script>
