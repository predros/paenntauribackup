<template>
  <v-group :config="shapeConfig.group">
    <v-image :config="shapeConfig.image" />
    <v-text :config="shapeConfig.text" />
  </v-group>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, defineProps, PropType, watch } from "vue";

import { INode, Direction } from "@/types/types";
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
});

const spring = computed(() => {
  if (props.direction == Direction.None) return 0;
  return props.node.springs[(props.direction as number) - 1];
});

const source = ref<HTMLImageElement>();

const text = computed<string>(() => {
  if (props.direction == Direction.Z)
    return settings.formatUnit(spring.value, UnitType.TorsionSpring);
  else return settings.formatUnit(spring.value, UnitType.Spring);
});

const shapeConfig = computed(() => {
  if (!source.value) return { image: null, text: null, group: null };
  else {
    let x = 0;
    let y = 0;
    switch (props.direction) {
      case Direction.X:
      case Direction.Y:
        x = 10;
        y = 0;
        break;
      case Direction.Z:
        x = 12;
        y = 14;
        break;
      default:
        break;
    }

    return {
      image: {
        image: source.value,
        offsetX: x,
        offsetY: y,
        listening: false,
      },
      text: {
        x: props.direction == Direction.Z ? 22 : 15,
        y: props.direction == Direction.Z ? 0 : 20,
        listening: false,
        fontFamily: "Roboto",
        text: text.value,
      },
      group: {
        x: props.node.x,
        y: -props.node.y,
        scaleX: 1 / props.scale,
        scaleY: 1 / props.scale,
        rotation: props.direction == Direction.X ? 90 : 0,
      },
    };
  }
});

function setImage(): void {
  const image = new window.Image();

  let path = "";
  switch (props.direction) {
    case Direction.None:
      return;
    case Direction.X:
    case Direction.Y:
      path = "Ky";
      break;
    case Direction.Z:
      path = "Kz";
      break;
  }
  image.src = new URL(
    `../../../assets/supports/springs${path}.svg`,
    import.meta.url,
  ).href;
  image.onload = () => (source.value = image);
}

watch(
  () => props.node,
  () => setImage(),
);

onMounted(() => {
  setImage();
});
</script>
