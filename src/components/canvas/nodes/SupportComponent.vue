<template>
  <v-image v-if="valid" :config="shapeConfig" />
</template>

<script setup lang="ts">
import { ref, onMounted, computed, defineProps } from "vue";
import { INode, SupportType } from "@/types/types";
import { watch } from "vue";

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
});

const valid = computed<boolean>(() => supportType.value != SupportType.None);

const supportType = computed<SupportType>(() => {
  const supportX = props.node.supports[0] ? 1 : 0;
  const supportY = props.node.supports[1] ? 2 : 0;
  const supportZ = props.node.supports[2] ? 4 : 0;

  return (supportX + supportY + supportZ) as SupportType;
});

const source = ref<HTMLImageElement>();

const shapeConfig = computed(() => {
  if (!source.value) return null;

  let x = 0;
  let y = 0;
  let angle = 0;

  switch (supportType.value) {
    case SupportType.None:
      return null;
    case SupportType.Rx:
      x = 12.5;
      y = 0;
      angle = 90;
      break;
    case SupportType.Ry:
      x = 12.5;
      y = 0;
      angle = 0;
      break;
    case SupportType.Rz:
      x = 10;
      y = 10;
      angle = 0;
      break;
    case SupportType.RxRy:
      x = 12.5;
      y = 0;
      angle = 0;
      break;
    case SupportType.RxRz:
      x = 19;
      y = 0;
      angle = 90;
      break;
    case SupportType.RyRz:
      x = 19;
      y = 0;
      angle = 0;
      break;
    case SupportType.RxRyRz:
      x = 19;
      y = 0;
      angle = 0;
      break;
  }

  const result = {
    x: props.node.x,
    y: -props.node.y,
    rotation: -props.node.support_angle + angle,
    scaleX: 1 / props.scale,
    scaleY: 1 / props.scale,
    image: source.value,
    offsetX: x,
    offsetY: y,
    listening: false,
  };
  return result;
});

watch(
  () => props.node,
  () => setImage(),
);

function setImage(): void {
  const image = new window.Image();
  let path = "";
  switch (supportType.value) {
    case SupportType.None:
      return;
    case SupportType.Rx:
    case SupportType.Ry:
      path = "Ry";
      break;
    case SupportType.Rz:
      path = "Rz";
      break;
    case SupportType.RxRy:
      path = "RxRy";
      break;
    case SupportType.RxRz:
    case SupportType.RyRz:
      path = "RyRz";
      break;
    case SupportType.RxRyRz:
      path = "RxRyRz";
      break;
  }
  image.src = new URL(
    `../../../assets/supports/supports${path}.svg`,
    import.meta.url,
  ).href;
  image.onload = () => (source.value = image);
}

onMounted(() => {
  setImage();
});
</script>
