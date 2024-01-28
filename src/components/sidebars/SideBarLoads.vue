<template>
  <div>
    <v-card-title class="pb-8">
      {{ t("sidebars.loads.title") }}
    </v-card-title>

    <v-card-text>
      <v-form
        ref="form"
        validate-on="submit"
      >
        <v-row>
          <v-col>
            <v-checkbox
              v-model="load.isLinear"
              :label="t('sidebars.loads.isLinear')"
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col>
            <v-text-field
              v-model="load.qx0"
              :label="t('sidebars.loads.xStart')"
              :placeholder="`(${settings.getUnitName(UnitType.Load)})`"
              :rules="[validNumber]"
              @keydown.stop
            />
          </v-col>
          <v-col>
            <v-text-field
              v-model="load.qx1"
              density="comfortable"
              :label="t('sidebars.loads.xEnd')"
              :placeholder="`(${settings.getUnitName(UnitType.Load)})`"
              :disabled="!load.isLinear"
              :rules="[validNumber]"
              @keydown.stop
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col>
            <v-text-field
              v-model="load.qy0"
              density="comfortable"
              :label="t('sidebars.loads.yStart')"
              :placeholder="`(${settings.getUnitName(UnitType.Load)})`"
              :rules="[validNumber]"
              @keydown.stop
            />
          </v-col>
          <v-col>
            <v-text-field
              v-model="load.qy1"
              density="comfortable"
              :label="t('sidebars.loads.yEnd')"
              :placeholder="`(${settings.getUnitName(UnitType.Load)})`"
              :disabled="!load.isLinear"
              :rules="[validNumber]"
              @keydown.stop
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col>
            <v-select
              v-model="load.isGlobal"
              :items="[
                { title: t('sidebars.loads.local'), value: false },
                { title: t('sidebars.loads.global'), value: true },
              ]"
              :label="t('sidebars.loads.direction')"
            />
          </v-col>
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
import { useI18n } from "vue-i18n";
import { VForm } from "vuetify/components";
import { isValidNumber, parseNumber } from "@/helper/misc";
import { ClickType, SideBarType } from "@/types/types";
import useGlobalStore from "@/state/global";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";
import { UnitType } from "@/types/units";

const { t } = useI18n();
const store = useGlobalStore();
const members = useMemberStore();
const settings = useSettings();

const form = ref<VForm>();

const load = reactive({
  qx0: "",
  qy0: "",
  qx1: "",
  qy1: "",
  isLinear: false,
  isGlobal: false,
});

function validNumber(value: string): boolean | string {
  return isValidNumber(value, true, true) || t("errors.validNumber");
}

async function onSubmit(): Promise<void> {
  if (!form.value) {
    return;
  }

  const { valid } = await form.value.validate();

  if (valid) {
    const qx0Parsed = parseNumber(load.qx0);
    const qy0Parsed = parseNumber(load.qy0);
    const qx1Parsed = load.isLinear ? parseNumber(load.qx1) : qx0Parsed;
    const qy1Parsed = load.isLinear ? parseNumber(load.qy1) : qy0Parsed;

    members.applyLoads(
      qx0Parsed,
      qy0Parsed,
      qx1Parsed,
      qy1Parsed,
      load.isGlobal,
    );
  }
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
