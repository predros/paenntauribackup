<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :width="props.width"
    :height="props.height"
    viewBox="0 0 18 18"
    :aria-labelledby="props.iconName"
    role="presentation"
  >
    <title :id="props.iconName">{{ iconName }} icon</title>
    <g :fill="props.iconColor" :stroke="props.iconColor">
      <slot />
    </g>
  </svg>
</template>

<script setup lang="ts">
import { defineProps } from "vue";

const props = defineProps({
  icon: {
    type: String,
    default: "",
  },
  iconName: {
    type: String,
    default: "box",
  },
  width: {
    type: [Number, String],
    default: 24,
  },
  height: {
    type: [Number, String],
    default: 24,
  },
  iconColor: {
    type: String,
    default: "currentColor",
  },
});
</script>
