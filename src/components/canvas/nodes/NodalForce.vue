<template>
  <v-group :config="shapeConfig.group">
    <v-arrow :config="shapeConfig.arrow" />
    <v-text :config="shapeConfig.text" />
  </v-group>
</template>

<script setup lang="ts">
import { PropType, computed, defineProps } from "vue";
import { Direction, INode } from "@/types/types";
import { lerp } from "@/helper/math";
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
  direction: {
    type: Number as PropType<Direction>,
    default: Direction.None,
  },
  extrema: {
    type: Object as () => { min: number; max: number },
    default: () => ({ min: 0, max: 0 }),
  },
});

const force = computed<number>(() => {
  switch (props.direction) {
    case Direction.X:
      return props.node.fx;
    case Direction.Y:
      return props.node.fy;
    default:
      return 0;
  }
});

const text = computed<string>(() =>
  settings.formatUnit(Math.abs(force.value), UnitType.Force),
);

const shapeConfig = computed(() => {
  const length = lerp(
    { x: props.extrema.min, y: 50 },
    { x: props.extrema.max, y: 75 },
    Math.abs(force.value),
  );

  let angle = -props.node.forceAngle;
  if (props.direction == Direction.Y) {
    angle -= 90;
  }
  if (force.value < 0) {
    angle += 180;
  }
  angle %= 360;

  const textScale =
    (angle <= 0 && angle > -90) || (angle <= -270 && angle > -360) ? 1 : -1;

  return {
    arrow: {
      stroke: "#0D47A1",
      fill: "#0D47A1",
      strokeWidth: 3,
      points: [-length, 0, 0, 0],
      listening: false,
    },
    text: {
      x: -length,
      y: 13,
      offsetX: length,
      offsetY: 6,
      width: 2 * length,
      height: 12,
      listening: false,
      align: "center",
      text: text.value,
      scaleX: textScale,
      scaleY: textScale,
      fill: "#0D47A1",
    },
    group: {
      listening: false,
      x: props.node.x,
      y: -props.node.y,
      offsetX: 7,
      offsetY: 0,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
      rotation: angle,
    },
  };
});
</script>
