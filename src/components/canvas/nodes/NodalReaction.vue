<template>
  <v-group :config="shapeConfig.group">
    <v-arrow :config="shapeConfig.arrow" />
    <v-text :config="shapeConfig.text" />
  </v-group>
</template>

<script setup lang="ts">
import { computed, defineProps, PropType } from "vue";
import { INode, Direction, INodeReaction } from "@/types/types";
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
  direction: {
    type: Number as PropType<Direction>,
    default: Direction.None,
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

const reaction = computed<number>(() => {
  switch (props.direction) {
    case Direction.X:
      return props.reactions.rx;
    case Direction.Y:
      return props.reactions.ry;
    default:
      return 0;
  }
});

const text = computed<string>(() =>
  settings.formatUnit(Math.abs(reaction.value), UnitType.Force),
);

const shapeConfig = computed(() => {
  const length = 80;
  let angle = props.node.support_angle;
  if (props.direction == Direction.Y) angle -= 90;
  if (reaction.value < 0) angle += 180;
  angle %= 360;

  const textScale =
    (angle <= 0 && angle > -90) || (angle <= -270 && angle > -360) ? 1 : -1;

  return {
    arrow: {
      stroke: "red",
      fill: "red",
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
      fontFamily: "Roboto",
      text: text.value,
      scaleX: textScale,
      scaleY: textScale,
      fill: "red",
    },
    group: {
      listening: false,
      x: props.node.x,
      y: -props.node.y,
      offsetX: 15,
      offsetY: 0,
      scaleX: 1 / props.scale,
      scaleY: 1 / props.scale,
      rotation: angle,
    },
  };
});
</script>
