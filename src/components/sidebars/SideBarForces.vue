<template>
  <div>
    <v-card-title class="pb-8">
      {{ t("sidebars.forces.title") }}
    </v-card-title>

    <v-card-text>
      <v-form ref="form" validate-on="submit">
        <v-row>
          <v-text-field
            v-model="force.fx"
            :rules="[validNumber]"
            :label="t('sidebars.forces.forceX')"
            :placeholder="`(${settings.getUnitName(UnitType.Force)})`"
            density="comfortable"
            prepend-inner-icon="mdi-axis-arrow"
            @keydown.stop
          />
        </v-row>

        <v-row>
          <v-text-field
            v-model="force.fy"
            :rules="[validNumber]"
            :label="t('sidebars.forces.forceY')"
            :placeholder="`(${settings.getUnitName(UnitType.Force)})`"
            density="comfortable"
            prepend-inner-icon="mdi-axis-arrow"
            @keydown.stop
          />
        </v-row>

        <v-row>
          <v-text-field
            v-model="force.mz"
            :rules="[validNumber]"
            :label="t('sidebars.forces.forceZ')"
            :placeholder="`(${settings.getUnitName(UnitType.Moment)})`"
            density="comfortable"
            prepend-inner-icon="mdi-axis-x-rotate-counterclockwise"
            @keydown.stop
          />
        </v-row>

        <v-row>
          <v-text-field
            v-model="force.angle"
            :label="t('sidebars.forces.forceAngle')"
            :placeholder="`(${settings.getUnitName(UnitType.Angle)})`"
            density="comfortable"
            prepend-inner-icon="mdi-angle-acute"
            :rules="[validNumber]"
            @keydown.stop
          />
        </v-row>
      </v-form>

      <v-row class="pt-10 px-2">
        <v-btn block color="primary" @click="onSubmit">
          {{ t("buttons.applyToSelection") }}
        </v-btn>
      </v-row>
      <v-row class="pt-2 px-2">
        <v-btn block @click="onReset">
          {{ t("buttons.clear") }}
        </v-btn>
      </v-row>
      <v-row class="pt-2 px-2">
        <v-btn block @click="onClose">
          {{ t("buttons.close") }}
        </v-btn>
      </v-row>
    </v-card-text>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { VForm } from "vuetify/components";
import { ClickType, SideBarType } from "@/types/types";
import { isValidNumber, parseNumber } from "@/helper/misc";
import useGlobalStore from "@/state/global";
import useNodeStore from "@/state/nodes";
import useSettings from "@/state/settings";
import { UnitType } from "@/types/units";

const { t } = useI18n();

const store = useGlobalStore();
const nodes = useNodeStore();
const settings = useSettings();

const form = ref<VForm>();

const force = reactive({
  fx: "",
  fy: "",
  mz: "",
  angle: "",
});

function validNumber(value: string): boolean | string {
  return isValidNumber(value, true, true) || t("errors.validNumber");
}

async function onSubmit(): Promise<void> {
  if (!form.value) return;

  const { valid } = await form.value.validate();

  if (valid) {
    const fxParsed = parseNumber(force.fx);
    const fyParsed = parseNumber(force.fy);
    const mzParsed = parseNumber(force.mz);
    const angleParsed = parseNumber(force.angle);

    nodes.applyNodalForces(fxParsed, fyParsed, mzParsed, angleParsed);

    form.value.reset();
  }
}

function onReset(): void {
  if (!form.value) return;
  form.value.reset();
}

function onClose(): void {
  store.current.clickType = ClickType.Select;
  store.current.sideBarType = SideBarType.Select;
}
</script>
