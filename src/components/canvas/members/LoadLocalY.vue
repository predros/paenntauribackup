<template>
  <v-group :config="groupConfig">
    <v-arrow
      v-for="index in numArrows"
      :key="index"
      :config="arrowConfig(index)"
    />
    <v-line :config="lineConfig.start" />
    <v-line
      v-if="props.member.qy0 * props.member.qy1 < 0"
      :config="lineConfig.end"
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
      isGLobal: false,
      tSup: false,
      tInf: false,
    }),
  },
  scale: {
    type: Number,
    default: 1.0,
  },
  extrema: {
    type: Object as () => { min: number; max: number },
    default: () => ({ max: 0, min: 0 }),
  },
});

const heights = computed(() => {
  const { max, min } = props.extrema;
  let start = lerp(
    { x: min, y: 10 },
    { x: max, y: 25 },
    Math.abs(props.member.qy0),
  );
  let end = lerp(
    { x: min, y: 10 },
    { x: max, y: 25 },
    Math.abs(props.member.qy1),
  );

  start = floatEq(props.member.qy0, 0) ? 0 : start;
  end = floatEq(props.member.qy1, 0) ? 0 : end;

  start *= Math.sign(props.member.qy0);
  end *= Math.sign(props.member.qy1);

  return { start, end };
});

const text = computed(() => {
  let start: string | null = null;
  let end: string | null = null;

  if (props.member.qy0 != 0) {
    start = settings.formatUnit(Math.abs(props.member.qy0), UnitType.Load);
  }

  if (props.member.qy1 != 0 && props.member.qy1 != props.member.qy0) {
    end = settings.formatUnit(Math.abs(props.member.qy1), UnitType.Load);
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
        y:
          (heights.value.start + Math.sign(props.member.qy0) * 30) /
          props.scale,
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
        y:
          (heights.value.start + Math.sign(props.member.qy0) * 30) /
          props.scale,
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

    if (text.value.end !== null) {
      textEnd = {
        x: props.member.length / 2,
        y: (heights.value.end + Math.sign(props.member.qy1) * 30) / props.scale,
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
  }

  return { start: textStart, end: textEnd };
});

const lineConfig = computed(() => {
  let lineFirst = {};
  let lineSecond = {};

  if (props.member.qy0 * props.member.qy1 < 0) {
    const root =
      (props.member.length * Math.abs(props.member.qy0)) /
      (Math.abs(props.member.qy0) + Math.abs(props.member.qy1));

    lineFirst = {
      x: 0,
      y: (Math.sign(props.member.qy0) * 20) / props.scale,
      points: [0, heights.value.start, root, 0],
      fill: "#C62828",
      stroke: "#C62828",
      strokeWidth: 1.5,
      scaleY: 1 / props.scale,
    };

    lineSecond = {
      x: 0,
      y: (Math.sign(props.member.qy1) * 20) / props.scale,
      points: [root, 0, props.member.length, heights.value.end],
      fill: "#C62828",
      stroke: "#C62828",
      strokeWidth: 1.5,
      scaleY: 1 / props.scale,
    };
  } else {
    lineFirst = {
      x: 0,
      y: (Math.sign(props.member.qy0) * 20) / props.scale,
      points: [0, heights.value.start, props.member.length, heights.value.end],
      fill: "#C62828",
      stroke: "#C62828",
      strokeWidth: 1.5,
      scaleY: 1 / props.scale,
    };
  }

  return { start: lineFirst, end: lineSecond };
});

function arrowConfig(index: number) {
  const x = (props.member.length * (index - 1)) / (numArrows.value - 1);
  const load = lerp(
    { x: 0, y: props.member.qy0 },
    { x: props.member.length, y: props.member.qy1 },
    x,
  );
  const height = lerp(
    { x: props.member.qy0, y: heights.value.start },
    { x: props.member.qy1, y: heights.value.end },
    load,
  );

  return {
    x,
    y: (Math.sign(load) * 20) / props.scale,
    fill: "#C62828",
    stroke: "#C62828",
    strokeWidth: 1.5,
    points: [0, height, 0, 0],
    scaleX: 1 / props.scale,
    scaleY: 1 / props.scale,
    pointerLength: 7.5,
    pointerWidth: 7.5,
    listening: false,
  };
}
</script>
