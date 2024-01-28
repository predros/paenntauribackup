<template>
  <v-group :config="shapeConfig.group">
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
      supportAngle: 0,
      springs: [0, 0, 0],
      prescribedDisplacements: [0, 0, 0],
      fx: 0,
      fy: 0,
      mz: 0,
      forceAngle: 0,
    }),
  },
  scale: {
    type: Number,
    default: 1.0,
  },
});

const text = computed<string>(() =>
  settings.formatUnit(Math.abs(props.node.mz), UnitType.Moment),
);

const shapeConfig = computed(() => {
  const radius = 20;
  const diag = radius * Math.SQRT1_2;

  return {
    arrow: {
      stroke: "#0D47A1",
      fill: "#0D47A1",
      strokeWidth: 3,
      points: [
        diag,
        -diag,
        0,
        -radius,
        -diag,
        -diag,
        -radius,
        0,
        -diag,
        diag,
        0,
        radius,
        diag,
        diag,
      ],
      tension: 0.4,
      listening: false,
    },
    text: {
      x: props.node.mz > 0 ? 12 : 45,
      y: 25,
      listening: false,
      text: text.value,
      fill: "#0D47A1",
      scaleX: props.node.mz > 0 ? 1 : -1,
    },
    group: {
      listening: false,
      x: props.node.x,
      y: -props.node.y,
      scaleY: 1 / props.scale,
      scaleX: props.node.mz > 0 ? 1 / props.scale : -1 / props.scale,
    },
  };
});
</script>
