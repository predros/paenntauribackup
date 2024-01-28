<template>
  <v-group :config="groupConfig">
    <v-line :config="lineConfig" />

    <v-line
      v-if="text.start !== null"
      :config="endLinesConfig.start"
    />
    <v-line
      v-if="text.end !== null"
      :config="endLinesConfig.end"
    />

    <v-line
      v-if="text.critFirst !== null"
      :config="endLinesConfig.critFirst"
    />
    <v-line
      v-if="text.critSecond !== null"
      :config="endLinesConfig.critSecond"
    />

    <v-text
      v-if="text.start !== null"
      :config="textConfig.start"
    />
    <v-text
      v-if="text.end !== null"
      :config="textConfig.end"
    />
    <v-text
      v-if="text.critFirst !== null"
      :config="textConfig.critFirst"
    />
    <v-text
      v-if="text.critSecond !== null"
      :config="textConfig.critSecond"
    />

    <v-line
      v-if="props.selected"
      :config="selectedLineConfig"
    />
    <v-text
      v-if="text.selected !== null"
      :config="textConfig.selected"
    />
  </v-group>
</template>

<script setup lang="ts">
import { PropType, computed, defineProps } from "vue";
import { IMember, IMemberResult, ResultType } from "@/types/types";
import useSettings from "@/state/settings";
import { UnitType } from "@/types/units";
import { floatEq } from "@/helper/math";

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
  result: {
    type: Object as () => IMemberResult,
    default: () => ({
      id: 0,
      dx: [],
      dy: [],
      rz: [],
      normal: [],
      shear: [],
      moment: [],
      maxMoment: [],
      minMoment: [],
      vertShear: [],
    }),
  },
  scale: {
    type: Number,
    default: 1.0,
  },
  resultType: {
    type: Number as PropType<ResultType>,
    default: ResultType.Displacement,
  },
  resultScale: {
    type: Number,
    default: 1.0,
  },
  extrema: {
    type: Object as () => {
      normal: number;
      shear: number;
      moment: number;
    },
    default: () => ({
      normal: 1,
      shear: 1,
      moment: 1,
    }),
  },
  selected: {
    type: Boolean,
    default: false,
  },
  selectedPosition: {
    type: Number,
    default: 0,
  },
});

const text = computed(() => {
  let start: number | undefined;
  let end: number | undefined;

  const result = {
    end: null as null | string,
    critFirst: null as null | string,
    critSecond: null as null | string,
    selected: null as null | string,
    start: null as null | string,
  };

  switch (props.resultType) {
    case ResultType.Displacement:
      break;
    case ResultType.Moment:
      start = props.result.moment.at(0);
      end = props.result.moment.at(-1);

      if (start == undefined || end == undefined) {
        return result;
      }

      result.start = floatEq(start, 0)
        ? null
        : settings.formatUnit(Math.abs(start), UnitType.Moment, false);
      result.end = floatEq(end, 0)
        ? null
        : settings.formatUnit(Math.abs(end), UnitType.Moment, false);

      if (props.result.minMoment[0] > 0) {
        result.critFirst = settings.formatUnit(
          Math.abs(props.result.minMoment[1]),
          UnitType.Moment,
          false,
        );
      }

      if (props.result.maxMoment[0] > 0) {
        result.critSecond = settings.formatUnit(
          Math.abs(props.result.maxMoment[1]),
          UnitType.Moment,
          false,
        );
      }

      break;
    case ResultType.Normal:
      start = props.result.normal.at(0);
      end = props.result.normal.at(-1);

      if (start == undefined || end == undefined) {
        return result;
      }

      result.start = floatEq(start, 0)
        ? null
        : settings.formatUnit(start, UnitType.Force, false);
      result.end = floatEq(end, 0)
        ? null
        : settings.formatUnit(end, UnitType.Force, false);

      if (props.result.vertNormal[0] > 0) {
        result.critFirst = settings.formatUnit(
          props.result.vertNormal[1],
          UnitType.Force,
          false,
        );
      }
      break;
    case ResultType.Shear:
      start = props.result.shear.at(0);
      end = props.result.shear.at(-1);

      if (start == undefined || end == undefined) {
        return result;
      }

      result.start = floatEq(start, 0)
        ? null
        : settings.formatUnit(start, UnitType.Force, false);
      result.end = floatEq(end, 0)
        ? null
        : settings.formatUnit(end, UnitType.Force, false);

      if (props.result.vertShear[0] > 0) {
        result.critFirst = settings.formatUnit(
          props.result.vertShear[1],
          UnitType.Force,
          false,
        );
      }
      break;
  }

  if (selectedProperties.value !== null) {
    const { index } = selectedProperties.value;

    switch (props.resultType) {
      case ResultType.Displacement:
        {
          const dx = settings.lengthFromCm(props.result.dx[index]);
          const dy = settings.lengthFromCm(props.result.dy[index]);
          const rz = props.result.rz[index];

          const formatDx = settings.formatUnit(dx, UnitType.Displacement, true);
          const formatDy = settings.formatUnit(dy, UnitType.Displacement, true);
          const formatRz = settings.formatUnit(rz, UnitType.Rotation, true);

          result.selected = `x: ${formatDx}\ny: ${formatDy}\nz: ${formatRz})`;
        }
        break;
      case ResultType.Moment:
        result.selected = settings.formatUnit(
          Math.abs(props.result.moment[index]),
          UnitType.Moment,
          false,
        );
        break;
      case ResultType.Normal:
        result.selected = settings.formatUnit(
          props.result.normal[index],
          UnitType.Force,
          false,
        );
        break;
      case ResultType.Shear:
        result.selected = settings.formatUnit(
          props.result.shear[index],
          UnitType.Force,
          false,
        );
        break;
    }
  }

  return result;
});

const generalProperties = computed(() => {
  const result = {
    start: undefined as number | undefined,
    end: undefined as number | undefined,
    critFirst: {
      x: undefined as number | undefined,
      y: undefined as number | undefined,
    },
    critSecond: {
      x: undefined as number | undefined,
      y: undefined as number | undefined,
    },
    invert: 1,
    color: "magenta",
    scale: 1,
  };

  switch (props.resultType) {
    case ResultType.Displacement:
      result.scale = 25 * props.resultScale;
      return result;
    case ResultType.Moment:
      result.start = props.result.moment.at(0);
      result.end = props.result.moment.at(-1);
      result.color = "green";
      result.invert = -1;
      result.scale = (100 * props.resultScale) / props.extrema.moment;

      if (text.value.critFirst !== null) {
        result.critFirst.x = props.result.minMoment[0];
        result.critFirst.y = props.result.minMoment[1];
      }

      if (text.value.critSecond !== null) {
        result.critSecond.x = props.result.maxMoment[0];
        result.critSecond.y = props.result.maxMoment[1];
      }
      break;
    case ResultType.Normal:
      result.start = props.result.normal.at(0);
      result.end = props.result.normal.at(-1);
      result.scale = (100 * props.resultScale) / props.extrema.normal;
      result.color = "blue";
      break;
    case ResultType.Shear:
      result.start = props.result.shear.at(0);
      result.end = props.result.shear.at(-1);
      result.scale = (100 * props.resultScale) / props.extrema.shear;
      result.color = "red";

      if (text.value.critFirst !== null) {
        result.critFirst.x = props.result.vertShear[0];
        result.critFirst.y = props.result.vertShear[1];
      }
      break;
  }

  return result;
});

const selectedProperties = computed(() => {
  if (!props.selected) {
    return null;
  }

  const numPoints = props.result.dx.length;
  const step = props.member.length / (numPoints - 1);

  const index = Math.round(props.selectedPosition / step);
  const x = index * step;

  let height = 0;
  let length = 0;
  switch (props.resultType) {
    case ResultType.Displacement:
      height = props.result.dy[index];
      length = props.result.dx[index];
      break;
    case ResultType.Moment:
      height = props.result.moment[index];
      break;
    case ResultType.Normal:
      height = props.result.normal[index];
      break;
    case ResultType.Shear:
      height = props.result.shear[index];
      break;
  }

  return {
    x,
    height,
    length,
    index,
  };
});

const groupConfig = computed(() => ({
  x: props.member.x0,
  y: -props.member.y0,
  rotation: -props.member.angle,
  listening: false,
}));

const lineConfig = computed(() => {
  let listY: number[];
  let listX: number[];

  switch (props.resultType) {
    case ResultType.Displacement:
      listY = props.result.dy;
      listX = props.result.dx;
      break;
    case ResultType.Moment:
      listY = props.result.moment;
      break;
    case ResultType.Normal:
      listY = props.result.normal;
      break;
    case ResultType.Shear:
      listY = props.result.shear;
      break;
  }
  const numPoints = listY.length;

  const scale = generalProperties.value.scale;
  const step = props.member.length / (numPoints - 1);
  const points: number[] = [];

  let x = 0;
  if (props.resultType == ResultType.Displacement) {
    listY.forEach((value, index) => {
      const valueX = listX[index];
      points.push(x + scale * valueX);
      points.push(-scale * value);

      x += step;
    });
  } else {
    listY.forEach((value) => {
      points.push(x);
      points.push(-scale * generalProperties.value.invert * value);

      x += step;
    });
  }
  return {
    x: 0,
    y: 0,
    points,
    stroke: generalProperties.value.color,
    strokeWidth: 1.25 / props.scale,
  };
});

const endLinesConfig = computed(() => {
  const result = {
    start: {},
    end: {},
    critFirst: {},
    critSecond: {},
  };

  const { start, end, invert, color, critFirst, critSecond, scale } =
    generalProperties.value;

  if (start == undefined) {
    return result;
  }
  if (text.value.start !== null) {
    const height = scale * invert * start;
    result.start = {
      x: 0,
      y: 0,
      points: [0, 0, 0, -height],
      strokeWidth: 1.25 / props.scale,
      stroke: color,
    };
  }

  if (end == undefined) {
    return result;
  }
  if (text.value.end !== null) {
    const height = scale * invert * end;

    result.end = {
      x: 0,
      y: 0,
      points: [props.member.length, 0, props.member.length, -height],
      strokeWidth: 1.25 / props.scale,
      stroke: generalProperties.value.color,
    };
  }

  if (critFirst.x != undefined && critFirst.y != undefined) {
    const height = scale * invert * critFirst.y;

    result.critFirst = {
      x: 0,
      y: 0,
      points: [critFirst.x, 0, critFirst.x, -height],
      strokeWidth: 1.25 / props.scale,
      stroke: color,
    };
  }

  if (critSecond.x != undefined && critSecond.y != undefined) {
    const height = scale * invert * critSecond.y;

    result.critSecond = {
      x: 0,
      y: 0,
      points: [critSecond.x, 0, critSecond.x, -height],
      strokeWidth: 1.25 / props.scale,
      stroke: color,
    };
  }

  return result;
});

const selectedLineConfig = computed(() => {
  if (!selectedProperties.value) {
    return {};
  }

  const { invert, scale } = generalProperties.value;
  let { height, length } = selectedProperties.value;
  const { x } = selectedProperties.value;

  height *= invert * scale;
  length *= invert * scale;

  return {
    x: 0,
    y: 0,
    points: [x, 0, x + length, -height],
    strokeWidth: 1.25 / props.scale,
    stroke: "red",
  };
});

const textConfig = computed(() => {
  const result = {
    start: {},
    end: {},
    critFirst: {},
    critSecond: {},
    selected: {},
  };

  const { start, end, invert, color, critFirst, critSecond, scale } =
    generalProperties.value;

  if (text.value.selected !== null && selectedProperties.value !== null) {
    const textScale =
      (props.member.angle >= 0 && props.member.angle < 90) ||
      (props.member.angle >= 270 && props.member.angle < 360)
        ? 1
        : -1;

    let { height, length } = selectedProperties.value;
    const { x } = selectedProperties.value;

    height *= invert * scale;
    length *= invert * scale;

    result.selected = {
      x: x + length,
      y: -height - (Math.sign(height) * 10) / props.scale,
      offsetX: (props.member.length * props.scale) / 2,
      offsetY: 20,
      width: props.member.length * props.scale,
      height: 40,
      align: "center",
      verticalAlign: "center",
      text: text.value.selected,
      fill: "red",
      listening: false,
      scaleX: textScale / props.scale,
      scaleY: textScale / props.scale,
    };
  }

  if (start == undefined) {
    return result;
  }
  if (text.value.start !== null) {
    const height = scale * invert * start;

    result.start = {
      x: 0,
      y: -height - (Math.sign(height) * 10) / props.scale,
      offsetX: (props.member.length * props.scale) / 2,
      offsetY: 6,
      width: props.member.length * props.scale,
      height: 12,
      align: "center",
      verticalAlign: "center",
      text: text.value.start,
      fill: color,
      listening: false,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
    };
  }

  if (end == undefined) {
    return result;
  }
  if (text.value.end !== null) {
    const height = scale * invert * end;

    result.end = {
      x: props.member.length,
      y: -height - (Math.sign(height) * 10) / props.scale,
      offsetX: (props.member.length * props.scale) / 2,
      offsetY: 6,
      width: props.member.length * props.scale,
      height: 12,
      align: "center",
      verticalAlign: "center",
      text: text.value.end,
      fill: color,
      listening: false,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
    };
  }

  if (critFirst.x != undefined && critFirst.y != undefined) {
    const height = scale * invert * critFirst.y;

    result.critFirst = {
      x: critFirst.x,
      y: -height - (Math.sign(height) * 10) / props.scale,
      offsetX: (props.member.length * props.scale) / 2,
      offsetY: 6,
      width: props.member.length * props.scale,
      height: 12,
      align: "center",
      verticalAlign: "center",
      text: text.value.critFirst,
      fill: color,
      listening: false,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
    };
  }

  if (critSecond.x != undefined && critSecond.y != undefined) {
    const height = scale * invert * critSecond.y;

    result.critSecond = {
      x: critSecond.x,
      y: -height - (Math.sign(height) * 10) / props.scale,
      offsetX: (props.member.length * props.scale) / 2,
      offsetY: 6,
      width: props.member.length * props.scale,
      height: 12,
      align: "center",
      verticalAlign: "center",
      text: text.value.critSecond,
      fill: color,
      listening: false,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
    };
  }
  return result;
});
</script>
