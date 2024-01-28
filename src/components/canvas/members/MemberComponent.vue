<template>
  <v-group :config="groupConfig">
    <v-line
      :config="lineConfig"
      @mousedown="onMouseDown"
      @mouseup="onMouseUp"
    />

    <v-circle
      v-if="props.member.hinges.start"
      :config="hingesConfig.start"
      @mousedown="onMouseDown"
      @mouseup="onMouseUp"
    />
    <v-circle
      v-if="props.member.hinges.end"
      :config="hingesConfig.end"
      @mousedown="onMouseDown"
      @mouseup="onMouseUp"
    />

    <v-text
      v-if="text.start !== null"
      :config="textConfig.start"
    />
    <v-line
      v-if="props.member.tSup != 0"
      :config="tempLinesConfig.start"
    />

    <v-text
      v-if="text.end !== null"
      :config="textConfig.end"
    />
    <v-line
      v-if="props.member.tInf != 0"
      :config="tempLinesConfig.end"
    />
  </v-group>
</template>

<script setup lang="ts">
import { computed, defineEmits, defineProps } from "vue";
import { IMember, KonvaMouseEvent } from "@/types/types";
import useSettings from "@/state/settings";
import { UnitType } from "@/types/units";

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
      tSup: 0,
      tInf: 0,
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

const groupConfig = computed(() => ({
  x: props.member.x0,
  y: -props.member.y0,
  rotation: -props.member.angle,
}));

const text = computed(() => {
  let start: string | null = null;
  let end: string | null = null;

  if (props.member.tSup != 0) {
    start = settings.formatUnit(props.member.tSup, UnitType.Temperature);
  }

  if (props.member.tInf != 0 && props.member.tSup != props.member.tInf) {
    end = settings.formatUnit(props.member.tInf, UnitType.Temperature);
  }

  return { start, end };
});

const tempColors = computed(() => {
  let colorSup: string | null;
  let colorInf: string | null;

  if (props.member.tSup == props.member.tInf) {
    colorSup = colorInf = props.member.tSup > 0 ? "#EF5350" : "#536DFE";
  } else if (props.member.tSup * props.member.tInf > 0) {
    if (props.member.tSup > 0) {
      colorSup = props.member.tSup > props.member.tInf ? "#B71C1C" : "#EF5350";
      colorInf = props.member.tSup > props.member.tInf ? "#EF5350" : "#B71C1C";
    } else {
      colorSup = props.member.tSup < props.member.tInf ? "#283593" : "#536DFE";
      colorInf = props.member.tSup < props.member.tInf ? "#536DFE" : "#283593";
    }
  } else {
    if (props.member.tSup == 0) {
      colorSup = null;
    } else {
      colorSup = props.member.tSup > 0 ? "#EF5350" : "#536DFE";
    }

    if (props.member.tInf == 0) {
      colorInf = null;
    } else {
      colorInf = props.member.tInf > 0 ? "#EF5350" : "#536DFE";
    }
  }

  return { sup: colorSup, inf: colorInf };
});

const lineConfig = computed(() => ({
  x: 0,
  y: 0,
  offsetY: 0,
  points: [0, 0, props.member.length, 0],
  stroke: props.selected ? "#F57C00" : "#4682B4",
  strokeWidth: 4,
  scaleY: 1 / props.scale,
  hitStrokeWidth: 10,
}));

const hingesConfig = computed(() => {
  let hingeStart = {};
  let hingeEnd = {};

  if (props.member.hinges.start) {
    hingeStart = {
      x: 0,
      y: 0,
      offsetX: -13,
      radius: 6,
      fill: "white",
      stroke: props.selected ? "#B71C1C" : "black",
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
      hitStrokeWidth: 8,
    };
  }

  if (props.member.hinges.end) {
    hingeEnd = {
      x: props.member.length,
      y: 0,
      offsetX: 13,
      radius: 6,
      fill: "white",
      stroke: props.selected ? "#B71C1C" : "black",
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
      hitStrokeWidth: 6,
    };
  }

  return { start: hingeStart, end: hingeEnd };
});

const tempLinesConfig = computed(() => {
  let lineStart = {};
  let lineEnd = {};
  if (props.member.tSup != 0) {
    lineStart = {
      x: 0,
      y: 0,
      offsetY: 5,
      points: [0, 0, props.member.length, 0],
      dash: [10, 5],
      stroke: tempColors.value.sup,
      strokeWidth: 1,
      scaleY: 1 / props.scale,
      listening: false,
    };
  }

  if (props.member.tInf != 0) {
    lineEnd = {
      x: 0,
      y: 0,
      offsetY: -5,
      points: [0, 0, props.member.length, 0],
      dash: [10, 5],
      stroke: tempColors.value.inf,
      strokeWidth: 1,
      scaleY: 1 / props.scale,
      listening: false,
    };
  }

  return { start: lineStart, end: lineEnd };
});

const textConfig = computed(() => {
  const textScale =
    (props.member.angle >= 0 && props.member.angle < 90) ||
    (props.member.angle >= 270 && props.member.angle < 360)
      ? 1
      : -1;
  let textStart = {};
  let textEnd = {};

  let align = "center";
  if (props.member.qx0 != 0 && props.member.qx0 == props.member.qx1) {
    align = textScale < 0 ? "right" : "left";
  }

  if (text.value.start !== null) {
    textStart = {
      x: props.member.length / 2,
      y: -15 / props.scale,
      offsetX: (props.member.length * props.scale) / 2,
      offsetY: 6,
      width: props.member.length * props.scale,
      height: 12,
      align,
      verticalAlign: "center",
      text: text.value.start,
      fill: tempColors.value.sup,
      listening: false,
      scaleX: textScale / props.scale,
      scaleY: textScale / props.scale,
    };
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
      align,
      verticalAlign: "center",
      fill: tempColors.value.inf,
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

function onMouseUp(e: KonvaMouseEvent): void {
  e.cancelBubble = true;
  emit("clicked", props.member.id);
}

function onMouseDown(e: KonvaMouseEvent): void {
  e.cancelBubble = true;
}
</script>
