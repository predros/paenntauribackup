<template>
  <v-group>
    <v-line
      v-for="index in numberLines.verticalNumber"
      :key="index"
      :config="getVerticalLineConfig(index)"
    />
    <v-line
      v-for="index in numberLines.horizontalNumber"
      :key="index"
      :config="getHorizontalLineConfig(index)"
    />
  </v-group>
</template>

<script setup lang="ts">
import { computed } from "vue";
import useGlobalStore from "@/state/global";
import useSettings from "@/state/settings";
import { IRectangle } from "@/helper/math";

const store = useGlobalStore();
const settings = useSettings();

const viewPortBounds = computed<IRectangle>(() => store.canvasProps.viewPortBounds);
const spacing = computed(() => settings.gridSpacing);

const numberLines = computed(() => {
  const x = viewPortBounds.value.x;
  const y = viewPortBounds.value.y;

  const sx = spacing.value.x;
  const sy = spacing.value.y;

  return {
    horizontalNumber: Math.floor(viewPortBounds.value.height / sy) + 2,
    verticalNumber: Math.floor(viewPortBounds.value.width / sx) + 2,
    horizontalFirst: y + (-y % sy),
    verticalFirst: x + (-x % sx),
  };
});

function getVerticalLineConfig(index: number): object {
  const y = viewPortBounds.value.y;
  const x = numberLines.value.verticalFirst + (index - 1) * spacing.value.x;

  return {
    x,
    y,
    points: [0, 0, 0, viewPortBounds.value.height],
    scaleX: 1 / store.canvasProps.scale,
    stroke: "lightGray",
    strokeWidth: 1.5,
    shadowForStrokeEnabled: false,
    listening: false,
  };
}

function getHorizontalLineConfig(index: number): object {
  const x = viewPortBounds.value.x;
  const y = numberLines.value.horizontalFirst + (index - 1) * spacing.value.y;

  return {
    x,
    y,
    points: [0, 0, viewPortBounds.value.width, 0],
    scaleY: 1 / store.canvasProps.scale,
    stroke: "lightGray",
    strokeWidth: 1.5,
    shadowForStrokeEnabled: false,
    listening: false,
  };
}
</script>
