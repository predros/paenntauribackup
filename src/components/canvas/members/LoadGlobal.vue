<template>
  <v-group :config="groupConfig">
    <v-line :config="shapeConfig.lineFirst" />
    <v-line v-if="root != null" :config="shapeConfig.lineSecond" />
    <v-arrow
      v-for="index in numArrows"
      :key="index"
      :config="arrowConfig(index)"
    />

    <v-text v-if="text.start != null" :config="shapeConfig.textStart" />
    <v-text v-if="text.end != null" :config="shapeConfig.textEnd" />
  </v-group>
</template>

<script setup lang="ts">
//#region Imports
import { computed, defineProps } from "vue";
import { IMember } from "@/types/types";
import { distance, lerp, floatEq } from "@/helper/math";
import { UnitType } from "@/types/units";

import useSettings from "@/state/settings";
//#endregion

//#region Store declarations
const settings = useSettings();
//#endregion

//#region Props and emits
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
      is_global: false,
      t_sup: false,
      t_inf: false,
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
//#endregion

//#region Computed properties
/**
 * Computes the current number of load arrows, depending on the member length and canvas scale.
 */
const numArrows = computed(() => {
  let result = Math.min(
    Math.floor(props.member.length / 8),
    Math.floor((props.member.length * props.scale) / 50),
  );
  result = Math.max(result, 2);
  return result;
});

/**
 * Computes, if it exists, the point where the load's y-direction is zero (otherwise, returns null).
 */
const root = computed<number | null>(() => {
  if (floatEq(loadVec.value.y.slope, 0)) return null;
  const root = -loadVec.value.y.intercept / loadVec.value.y.slope;

  if (root < 0 || root > props.member.length) return null;

  return root;
});

/**
 * Computes the lengths of the start and end arrows, along with the ratio between load and length
 * (to determine the other arrows' x and y components without needing to calculate their length).
 */
const heights = computed(() => {
  const { max, min } = props.extrema;
  const startResult = distance(
    { x: 0, y: 0 },
    { x: props.member.qx0, y: props.member.qy0 },
  );
  const endResult = distance(
    { x: 0, y: 0 },
    { x: props.member.qx1, y: props.member.qy1 },
  );

  const start =
    startResult != 0
      ? lerp({ x: min, y: 10 }, { x: max, y: 25 }, startResult)
      : 0;
  const end =
    endResult != 0 ? lerp({ x: min, y: 10 }, { x: max, y: 25 }, endResult) : 0;
  const maxHeight = Math.max(start, end);
  const maxLoad = Math.max(startResult, endResult);
  const maxRatio = maxHeight / maxLoad;

  return { start, end, maxRatio };
});

/**
 * Computes the parameters (slope and intercept) of the equations for the x and y loads (in local coordinates).
 */
const loadVec = computed(() => {
  const deltaX = props.member.x1 - props.member.x0;
  const deltaY = props.member.y1 - props.member.y0;
  const deltaQx = props.member.qx1 - props.member.qx0;
  const deltaQy = props.member.qy1 - props.member.qy0;

  const cos = deltaX / props.member.length;
  const sin = deltaY / props.member.length;

  const xIntercept = props.member.qy0 * sin + props.member.qx0 * cos;
  const yIntercept = -props.member.qy0 * cos + props.member.qx0 * sin;

  const xSlope = (deltaQy * sin + deltaQx * cos) / props.member.length;
  const ySlope = (-deltaQy * cos + deltaQx * sin) / props.member.length;

  return {
    x: {
      slope: xSlope,
      intercept: xIntercept,
    },
    y: {
      slope: ySlope,
      intercept: yIntercept,
    },
  };
});

/**
 * Computes the text to be used in the text labels.
 */
const text = computed(() => {
  let textStart: string | null = null;
  let textEnd: string | null = null;

  if (props.member.qx0 != 0 || props.member.qy0 != 0) {
    const resulting = distance(
      { x: 0, y: 0 },
      { x: props.member.qx0, y: props.member.qy0 },
    );
    textStart = settings.formatUnit(resulting, UnitType.Load);
  }

  if (
    (props.member.qx1 != 0 && props.member.qx0 != props.member.qx1) ||
    (props.member.qy1 != 0 && props.member.qy0 != props.member.qy1)
  ) {
    const resulting = distance(
      { x: 0, y: 0 },
      { x: props.member.qx1, y: props.member.qy1 },
    );
    textEnd = settings.formatUnit(resulting, UnitType.Load);
  }

  return {
    start: textStart,
    end: textEnd,
  };
});

/**
 * Computes the position and rotation of the whole group.
 */
const groupConfig = computed(() => ({
  x: props.member.x0,
  y: -props.member.y0,
  rotation: -props.member.angle,
  listening: false,
}));

/**
 * Computes the position and properties of the text labels and upper lines.
 */
const shapeConfig = computed(() => {
  let lineFirst = {};
  let lineSecond = {};
  let textStart = {};
  let textEnd = {};

  const start = {
    x: loadVec.value.x.intercept,
    y: loadVec.value.y.intercept,
  };
  const end = {
    x: loadVec.value.x.intercept + loadVec.value.x.slope * props.member.length,
    y: loadVec.value.y.intercept + loadVec.value.y.slope * props.member.length,
  };

  const lengthStart = {
    x: start.x * heights.value.maxRatio,
    y: start.y * heights.value.maxRatio,
  };

  const lengthEnd = {
    x: end.x * heights.value.maxRatio,
    y: end.y * heights.value.maxRatio,
  };

  const textScale =
    (props.member.angle >= 0 && props.member.angle < 90) ||
    (props.member.angle >= 270 && props.member.angle < 360)
      ? 1
      : -1;

  if (text.value.start != null) {
    if (text.value.end == null) {
      textStart = {
        x: props.member.length / 2,
        y: -(lengthStart.y + Math.sign(start.y) * 25) / props.scale,
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
        y: -(lengthStart.y + Math.sign(start.y) * 25) / props.scale,
        offsetX: (props.member.length * props.scale) / 2,
        offsetY: 6,
        width: props.member.length * props.scale,
        height: 12,
        text: text.value.start,
        align: textScale > 0 ? "left" : "right",
        verticalAlign: "center",
        fill: "#C62828",
        listening: false,
        scaleX: textScale / props.scale,
        scaleY: textScale / props.scale,
      };
    }
  }

  if (text.value.end != null) {
    textEnd = {
      x: props.member.length / 2,
      y: -(lengthEnd.y + Math.sign(end.y) * 25) / props.scale,
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

  if (lengthStart.y != 0 || lengthEnd.y != 0) {
    if (root.value != null) {
      lineFirst = {
        x: 0,
        y: (-Math.sign(start.y) * 15) / props.scale,
        points: [-lengthStart.x / props.scale, -lengthStart.y, root.value, 0],
        fill: "#C62828",
        stroke: "#C62828",
        strokeWidth: 1.5,
        scaleY: 1 / props.scale,
      };

      lineSecond = {
        x: 0,
        y: (-Math.sign(end.y) * 15) / props.scale,
        points: [
          root.value,
          0,
          props.member.length - lengthEnd.x / props.scale,
          -lengthEnd.y,
        ],
        fill: "#C62828",
        stroke: "#C62828",
        strokeWidth: 1.5,
        scaleY: 1 / props.scale,
      };
    } else {
      lineFirst = {
        x: 0,
        y: (-Math.sign(start.y) * 15) / props.scale,
        points: [
          -lengthStart.x / props.scale,
          -lengthStart.y,
          props.member.length - lengthEnd.x / props.scale,
          -lengthEnd.y,
        ],
        fill: "#C62828",
        stroke: "#C62828",
        strokeWidth: 1.5,
        scaleY: 1 / props.scale,
      };
    }
  }

  return {
    lineFirst,
    lineSecond,
    textStart,
    textEnd,
  };
});
//#endregion

//#region Functions
/**
 * Computes the properties of an arrow, given its index (from 1 to numArrows).
 * @param index The arrow's index.
 */
function arrowConfig(index: number): object {
  const x = (props.member.length * (index - 1)) / (numArrows.value - 1);
  const loadX = loadVec.value.x.intercept + loadVec.value.x.slope * x;
  const loadY = loadVec.value.y.intercept + loadVec.value.y.slope * x;

  let lengthX = loadX * heights.value.maxRatio;
  const lengthY = loadY * heights.value.maxRatio;

  if (floatEq(lengthY, 0)) {
    lengthX = 5;
    if (index == 1) return {};
  }

  return {
    x,
    y: (-Math.sign(loadY) * 15) / props.scale,
    fill: "#C62828",
    stroke: "#C62828",
    strokeWidth: 1.5,
    points: [-lengthX, -lengthY, 0, 0],
    scaleX: 1 / props.scale,
    scaleY: 1 / props.scale,
    pointerLength: 5,
    pointerWidth: 5,
    listening: false,
  };
}
//#endregion
</script>
