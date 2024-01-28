<template>
  <div>
    <v-card-title class="pb-10">
      {{ t("sidebars.temperature.title") }}
    </v-card-title>

    <v-card-text>
      <v-form
        ref="form"
        validate-on="submit"
      >
        <v-row>
          <v-text-field
            v-model="temperature.sup"
            density="comfortable"
            :rules="[validNumber]"
            :label="t('sidebars.temperature.superior')"
            :placeholder="`(${settings.getUnitName(UnitType.Temperature)})`"
            prepend-inner-icon="mdi-thermometer-plus"
          />
        </v-row>

        <v-row>
          <v-text-field
            v-model="temperature.inf"
            density="comfortable"
            :rules="[validNumber]"
            :label="t('sidebars.temperature.inferior')"
            :placeholder="`(${settings.getUnitName(UnitType.Temperature)})`"
            prepend-inner-icon="mdi-thermometer-minus"
          />
        </v-row>
      </v-form>

      <v-row class="pt-10 px-2">
        <v-btn
          block
          color="primary"
          @click="onSubmit"
        >
          {{ t("buttons.applyToSelection") }}
        </v-btn>
      </v-row>
      <v-row class="pt-2 px-2">
        <v-btn
          block
          @click="onReset"
        >
          {{ t("buttons.clear") }}
        </v-btn>
      </v-row>
      <v-row class="pt-2 px-2">
        <v-btn
          block
          @click="onClose"
        >
          {{ t("buttons.close") }}
        </v-btn>
      </v-row>
    </v-card-text>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from "vue";
import { VForm } from "vuetify/components";
import { ClickType, SideBarType } from "@/types/types";

import useGlobalStore from "@/state/global";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";

import { useI18n } from "vue-i18n";
import { isValidNumber, parseNumber } from "@/helper/misc";
import { UnitType } from "@/types/units";

const { t } = useI18n();
const store = useGlobalStore();
const members = useMemberStore();
const settings = useSettings();

const form = ref<VForm>();

const temperature = reactive({
  sup: "",
  inf: "",
});

async function onSubmit(): Promise<void> {
  if (!form.value) {
    return;
  }

  const { valid } = await form.value.validate();

  if (valid) {
    const tSupParsed = parseNumber(temperature.sup);
    const tInfParsed = parseNumber(temperature.inf);

    members.applyTemperatures(tSupParsed, tInfParsed);
  }
}

function validNumber(value: string): boolean | string {
  return isValidNumber(value) || t("errors.validNumber");
}

function onReset(): void {
  if (!form.value) {
    return;
  }
  form.value.reset();
}

function onClose(): void {
  store.current.clickType = ClickType.Select;
  store.current.sideBarType = SideBarType.Select;
}
</script>
