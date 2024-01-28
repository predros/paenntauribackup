<template>
  <v-group :config="groupConfig">
    <v-arrow
      v-for="index in numArrows"
      :key="index"
      :config="arrowConfig(index)"
    />
    <v-text
      v-if="text.start !== null"
      :config="textConfig.start"
    />
    <v-text
      v-if="text.end !== null"
      :config="textConfig.end"
    />
  </v-group>
</template>

<script setup lang="ts">
import { computed, defineProps } from "vue";
import { IMember } from "@/types/types";
import { floatEq, lerp } from "@/helper/math";
import { UnitType } from "@/types/units";
import useSettings from "@/state/settings";

const settings = useSettings();

const props = defineProps({
  member: {
    type: Object as () => IMember,
    default: () => ({
      id: 0,
      x0: 0,
      y0: 0,
      x1: 0,
      y1: 0,
      length: 0,
      angle: 0,
      hinges: { start: false, end: false },
      qx0: 0,
      qy0: 0,
      qx1: 0,
      qy1: 0,
      isGlobal: false,
      tSup: false,
      tInf: false,
    }),
  },
  scale: {
    type: Number,
    default: 1.0,
  },
});

const text = computed(() => {
  let start: string | null = null;
  let end: string | null = null;

  if (props.member.qx0 != 0) {
    start = settings.formatUnit(Math.abs(props.member.qx0), UnitType.Load);
  }

  if (props.member.qx1 != 0 && props.member.qx1 != props.member.qx0) {
    end = settings.formatUnit(Math.abs(props.member.qx1), UnitType.Load);
  }

  return { start, end };
});

const numArrows = computed(() => {
  let result = Math.min(
    Math.floor(props.member.length / 8),
    Math.floor((props.member.length * props.scale) / 50),
  );
  result = Math.max(result, 2);
  return result;
});

const groupConfig = computed(() => ({
  x: props.member.x0,
  y: -props.member.y0,
  rotation: -props.member.angle,
  listening: false,
}));

const textConfig = computed(() => {
  const textScale =
    (props.member.angle >= 0 && props.member.angle < 90) ||
    (props.member.angle >= 270 && props.member.angle < 360)
      ? 1
      : -1;
  let textStart = {};
  let textEnd = {};

  if (text.value.start !== null) {
    if (text.value.end === null) {
      textStart = {
        x: props.member.length / 2,
        y: 15 / props.scale,
        offsetX: (props.member.length * props.scale) / 2,
        offsetY: 6,
        width: props.member.length * props.scale,
        height: 12,
        align: "center",
        verticalAlign: "center",
        text: text.value.start,
        fill: "#C62828",
        listening: false,
        scaleX: textScale / props.scale,
        scaleY: textScale / props.scale,
      };
    } else {
      textStart = {
        x: props.member.length / 2,
        y: 15 / props.scale,
        offsetX: (props.member.length * props.scale) / 2,
        offsetY: 6,
        width: props.member.length * props.scale,
        height: 12,
        text: text.value.start,
        align: textScale < 0 ? "right" : "left",
        verticalAlign: "center",
        fill: "#C62828",
        listening: false,
        scaleX: textScale / props.scale,
        scaleY: textScale / props.scale,
      };
    }
  }

  if (text.value.end !== null) {
    textEnd = {
      x: props.member.length / 2,
      y: 15 / props.scale,
      offsetX: (props.member.length * props.scale) / 2,
      offsetY: 6,
      width: props.member.length * props.scale,
      height: 12,
      text: text.value.end,
      align: textScale > 0 ? "right" : "left",
      verticalAlign: "center",
      fill: "#C62828",
      listening: false,
      scaleX: textScale / props.scale,
      scaleY: textScale / props.scale,
    };
  }

  return {
    start: textStart,
    end: textEnd,
  };
});

function arrowConfig(index: number) {
  const x = (props.member.length * (index - 1)) / numArrows.value;
  const value = lerp(
    { x: 0, y: props.member.qx0 },
    { x: props.member.length, y: props.member.qx1 },
    x,
  );
  return floatEq(value, 0)
    ? {}
    : {
        x,
        y: 1,
        fill: "#C62828",
        stroke: "#C62828",
        strokeWidth: 1,
        points: value > 0 ? [0, 0, 10, 0] : [10, 0, 0, 0],
        pointerLength: 10,
        pointerWidth: 10,
        scaleX: 1 / props.scale,
        scaleY: 1 / props.scale,
        listening: false,
      };
}
</script>
