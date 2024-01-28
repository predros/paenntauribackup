<template>
  <v-card class="py-4 px-6">
    <v-card-title>{{ t("dialogs.combinationFactors.title") }} </v-card-title>

    <v-card-text>
      <v-form
        ref="form"
        validate-on="blur"
      >
        <v-table
          :height="300"
          :min-width="550"
          fixed-header
          density="compact"
        >
          <thead>
            <tr>
              <th
                class="text-left"
                style="min-width: 150px"
              ></th>
              <th
                class="text-center"
                v-for="loadcase in loadcases"
                :key="loadcase.id"
              >
                {{ loadcase.name }}
              </th>
            </tr>
          </thead>

          <tbody>
            <tr
              class="pa-0 ma-0"
              v-for="i in combinations.length"
              :key="i"
            >
              <td class="text-left full-height">
                {{ combinations[i - 1].name }}
              </td>
              <td
                class="full-height py-0 my-0"
                v-for="j in loadcases.length"
                :key="j"
              >
                <v-text-field
                  class="align-center pa-0 ma-0"
                  v-model="factors[i - 1][j - 1]"
                  :rules="[validNumber]"
                  variant="underlined"
                  density="comfortable"
                />
              </td>
            </tr>
          </tbody>
        </v-table>
      </v-form>
    </v-card-text>

    <div class="d-flex justify-end">
      <v-btn
        class="pl-3 pr-3 mr-3"
        color="primary"
        @click="onSubmitAndClose"
      >
        {{ t("buttons.save") }}
      </v-btn>
      <v-btn
        class="pl-3 mr-3 pr-3"
        @click="onSubmit"
      >
        {{ t("buttons.apply") }}
      </v-btn>

      <v-btn
        class="pl-3 pr-3"
        @close="onClose"
      >
        {{ t("buttons.close") }}
      </v-btn>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, defineEmits } from "vue";
import useGlobalStore from "@/state/global";
import { ICombination, ILoadcase } from "@/types/types";
import { isValidNumber, parseNumber } from "@/helper/misc";
import { VForm } from "vuetify/components";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const emit = defineEmits(["close"]);

const store = useGlobalStore();

const form = ref<VForm>();

const combinations = computed(() => {
  const result: ICombination[] = JSON.parse(
    JSON.stringify(store.combinationsList),
  );
  result.sort((a, b) => (a.name.toLowerCase() < b.name.toLowerCase() ? -1 : 1));

  return result;
});

const loadcases = computed(() => {
  const result: ILoadcase[] = JSON.parse(JSON.stringify(store.loadcasesList));
  result.sort((a, b) => (a.name.toLowerCase() < b.name.toLowerCase() ? -1 : 1));

  return result;
});

const factors = ref<string[][]>(
  Array.from(Array(store.combinationsList.length), () =>
    Array.from(Array(store.loadcasesList.length), () => ""),
  ),
);

onMounted(() => {
  combinations.value.forEach((comb, combIndex) => {
    const currentModel = factors.value[combIndex];
    const currentComb = comb.loadFactors;

    loadcases.value.forEach((loadcase, loadcaseIndex) => {
      const id = loadcase.id;
      currentModel[loadcaseIndex] = String(currentComb[id]);
    });
  });
});

async function onSubmit() {
  if (!form.value) {
    return;
  }

  const { valid } = await form.value.validate();

  if (valid) {
    const result: Record<number, Record<number, number>> = {};

    combinations.value.forEach((comb, combIndex) => {
      const currentFactors = factors.value[combIndex].map((x) =>
        parseNumber(x),
      );
      const loadFactors: Record<number, number> = {};

      loadcases.value.forEach((loadcase, loadcaseIndex) => {
        loadFactors[loadcase.id] = currentFactors[loadcaseIndex];
      });

      result[comb.id] = loadFactors;
    });

    await store.combinationApplyFactors(result);
  }
}

function onClose() {
  emit("close");
}

async function onSubmitAndClose() {
  await onSubmit();
  onClose();
}

function validNumber(value: string): boolean | string {
  return isValidNumber(value, true, true) || "";
}
</script>
