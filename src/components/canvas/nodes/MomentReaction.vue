<template>
  <v-group :config="shapeConfig.group">
    <v-arrow :config="shapeConfig.arrow" />
    <v-text :config="shapeConfig.text" />
  </v-group>
</template>

<script setup lang="ts">
import { computed, defineProps } from "vue";
import { INode, INodeReaction } from "@/types/types";
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
  reactions: {
    type: Object as () => INodeReaction,
    default: () => ({
      id: 0,
      rx: 0,
      ry: 0,
      mz: 0,
    }),
  },
});

const text = computed<string>(() =>
  settings.formatUnit(Math.abs(props.reactions.mz), UnitType.Moment),
);

const shapeConfig = computed(() => {
  const radius = 30;
  const diag = radius * Math.SQRT1_2;

  return {
    arrow: {
      stroke: "red",
      fill: "red",
      strokeWidth: 3,
      points: [
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
      x: props.reactions.mz > 0 ? 20 : 50,
      y: 35,
      listening: false,
      text: text.value,
      fill: "red",
      scaleX: props.reactions.mz > 0 ? 1 : -1,
    },
    group: {
      listening: false,
      x: props.node.x,
      y: -props.node.y,
      scaleY: 1 / props.scale,
      scaleX: props.reactions.mz > 0 ? 1 / props.scale : -1 / props.scale,
    },
  };
});
</script>
