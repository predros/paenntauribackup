<template>
  <v-group :config="shapeConfig.group">
    <v-arrow :config="shapeConfig.arrow" />
    <v-line :config="shapeConfig.dimLine.start" />
    <v-line :config="shapeConfig.dimLine.end" />
    <v-line :config="shapeConfig.dimLine.startOblique" />
    <v-line :config="shapeConfig.dimLine.endOblique" />
    <v-text :config="shapeConfig.text" />
  </v-group>
</template>

<script setup lang="ts">
import { PropType, computed, defineProps } from "vue";
import { Direction, INode } from "@/types/types";
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
});

const displacement = computed<number>(() => {
  switch (props.direction) {
    case Direction.X:
      return props.node.prescribedDisplacements[0];
    case Direction.Y:
      return props.node.prescribedDisplacements[1];
    default:
      return 0;
  }
});

const text = computed<string>(() =>
  settings.formatUnit(Math.abs(displacement.value), UnitType.Displacement),
);

const shapeConfig = computed(() => {
  let angle = -props.node.supportAngle;
  if (props.direction == Direction.Y) {
    angle -= 90;
  }
  if (displacement.value < 0) {
    angle += 180;
  }
  angle %= 360;

  const textScale =
    (angle <= 0 && angle > -90) || (angle <= -270 && angle > -360) ? 1 : -1;

  return {
    arrow: {
      stroke: "#004D40",
      fill: "#004D40",
      strokeWidth: 2,
      points: [0, 0, 50, 0],
      listening: false,
    },
    dimLine: {
      start: {
        stroke: "#004D40",
        points: [5, -8, 5, 8],
        listening: false,
      },
      end: {
        stroke: "#004D40",
        points: [15, -8, 15, 8],
        listening: false,
      },
      startOblique: {
        stroke: "#004D40",
        strokeWidth: 1,
        points: [10, -8, 0, 8],
        listening: false,
      },
      endOblique: {
        stroke: "#004D40",
        strokeWidth: 1,
        points: [20, -8, 10, 8],
        listening: false,
      },
    },
    text: {
      x: 50,
      y: 16,
      offsetX: 50,
      offsetY: 6,
      width: 100,
      height: 12,
      listening: false,
      align: "center",
      text: text.value,
      scaleX: textScale,
      scaleY: textScale,
      fill: "#004D40",
    },
    group: {
      listening: false,
      x: props.node.x,
      y: -props.node.y,
      offsetX: -4,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
      rotation: angle,
    },
  };
});
</script>
