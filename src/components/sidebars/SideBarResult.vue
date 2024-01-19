<template>
  <div>
    <v-card-title class="pb-8">
      {{ t("sidebars.result.title") }}
    </v-card-title>

    <v-card-text>
      <v-row>
        <v-select
          v-model="store.current.result"
          :label="t('sidebars.result.current')"
          :items="resultCases"
        />

        <v-select
          v-model="store.results.type"
          :label="t('sidebars.result.type')"
          :items="[
            {
              title: t('sidebars.result.displacement'),
              value: ResultType.Displacement,
            },
            { title: t('sidebars.result.normal'), value: ResultType.Normal },
            { title: t('sidebars.result.shear'), value: ResultType.Shear },
            { title: t('sidebars.result.moment'), value: ResultType.Moment },
          ]"
        />
      </v-row>

      <v-row>
        <v-slider
          v-model="scale"
          :label="t('sidebars.result.scale')"
          color="primary"
          append-icon="mdi-ruler"
        />
      </v-row>

      <v-row>
        <v-checkbox
          v-model="store.results.showReactions"
          :label="t('sidebars.result.showReactions')"
          color="primary"
        />
      </v-row>

      <v-row class="pt-10 px-2">
        <v-btn block color="primary" @click="onClose">
          {{ t("buttons.close") }}
        </v-btn>
      </v-row>
    </v-card-text>
  </div>
</template>

<script setup lang="ts">
import { ClickType, ResultType, SideBarType } from "@/types/types";
import { useI18n } from "vue-i18n";
import { ref, watch, computed } from "vue";

import useGlobalStore from "@/state/global";

const { t } = useI18n();
const store = useGlobalStore();

const scale = ref<number>(25.0);

const resultCases = computed(() => {
  const result = [] as Array<{
    title: string;
    value: { isCombination: boolean; id: number };
  }>;

  store.loadcasesList.forEach((loadcase) => {
    result.push({
      title: t("sidebars.result.loadcase") + " " + loadcase.name,
      value: { isCombination: false, id: loadcase.id },
    });
  });

  store.combinationsList.forEach((combination) => {
    result.push({
      title: t("sidebars.result.combination") + " " + combination.name,
      value: { isCombination: true, id: combination.id }
    });
  })

  return result;
});

function onClose(): void {
  store.current.clickType = ClickType.Select;
  store.current.sideBarType = SideBarType.Select;
}

watch(scale, () => {
  store.results.scale = Math.exp(0.0326902 * scale.value - 0.234498) - 0.790968;
});
</script>
