<template>
  <div>
    <v-card-title class="pb-3">
      {{ t("sidebars.supports.title") }}
    </v-card-title>

    <v-card-text>
      <v-form
        ref="form"
        validate-on="submit"
      >
        <v-row class="pb-6">
          <v-tabs
            v-model="tab"
            background-color="primary"
          >
            <v-tab :value="0">{{ t("sidebars.supports.labelSupports") }}</v-tab>
            <v-tab :value="1">{{ t("sidebars.supports.labelSprings") }}</v-tab>
            <v-tab :value="2">{{
              t("sidebars.supports.labelDisplacements")
            }}</v-tab>
          </v-tabs>
        </v-row>

        <v-window
          v-model="tab"
          class="pt-3"
        >
          <v-window-item :value="0">
            <h3 class="pb-1">{{ t("sidebars.supports.titleSupports") }}</h3>
            <v-checkbox
              v-model="restr.x"
              :label="t('sidebars.supports.supportX')"
              density="comfortable"
            />
            <v-checkbox
              v-model="restr.y"
              :label="t('sidebars.supports.supportY')"
              density="comfortable"
            />
            <v-checkbox
              v-model="restr.z"
              :label="t('sidebars.supports.supportZ')"
              density="comfortable"
            />
            <v-text-field
              v-model="restr.angle"
              :label="t('sidebars.supports.supportAngle')"
              :placeholder="`(${settings.getUnitName(UnitType.Angle)})`"
              prepend-inner-icon="mdi-angle-acute"
              density="comfortable"
              :rules="[validNumber]"
            />
          </v-window-item>

          <v-window-item :value="1">
            <h3 class="pb-5">{{ t("sidebars.supports.titleSprings") }}</h3>
            <v-text-field
              v-model="springs.x"
              :label="t('sidebars.supports.springX')"
              :placeholder="`(${settings.getUnitName(UnitType.Spring)})`"
              prepend-inner-icon="mdi-axis-arrow"
              density="comfortable"
              :rules="[validNumber, nonNegative]"
              :disabled="restr.x"
              @keydown.stop
            />
            <v-text-field
              v-model="springs.y"
              :label="t('sidebars.supports.springY')"
              :placeholder="`(${settings.getUnitName(UnitType.Spring)})`"
              prepend-inner-icon="mdi-axis-arrow"
              density="comfortable"
              :rules="[validNumber, nonNegative]"
              :disabled="restr.y"
              @keydown.stop
            />
            <v-text-field
              v-model="springs.z"
              :label="t('sidebars.supports.springZ')"
              :placeholder="`(${settings.getUnitName(UnitType.TorsionSpring)})`"
              prepend-inner-icon="mdi-axis-x-rotate-counterclockwise"
              density="comfortable"
              :rules="[validNumber, nonNegative]"
              :disabled="restr.z"
              @keydown.stop
            />
          </v-window-item>

          <v-window-item :value="2">
            <h3 class="pb-5">
              {{ t("sidebars.supports.titleDisplacements") }}
            </h3>
            <v-text-field
              v-model="displacement.x"
              :label="t('sidebars.supports.displacementX')"
              :placeholder="`(${settings.getUnitName(UnitType.Displacement)})`"
              prepend-inner-icon="mdi-axis-arrow"
              density="comfortable"
              :rules="[validNumber]"
              :disabled="!restr.x"
              @keydown.stop
            />
            <v-text-field
              v-model="displacement.y"
              :label="t('sidebars.supports.displacementY')"
              :placeholder="`(${settings.getUnitName(UnitType.Displacement)})`"
              prepend-inner-icon="mdi-axis-arrow"
              density="comfortable"
              :rules="[validNumber]"
              :disabled="!restr.y"
              @keydown.stop
            />
            <v-text-field
              v-model="displacement.z"
              :label="t('sidebars.supports.displacementZ')"
              :placeholder="`(${settings.getUnitName(UnitType.Rotation)})`"
              prepend-inner-icon="mdi-axis-x-rotate-counterclockwise"
              density="comfortable"
              :rules="[validNumber]"
              :disabled="!restr.z"
              @keydown.stop
            />
          </v-window-item>
        </v-window>
      </v-form>

      <v-row class="pt-8 px-2">
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
import { isValidNumber, parseNumber } from "@/helper/misc";
import { ClickType, SideBarType } from "@/types/types";
import useNodeStore from "@/state/nodes";
import useGlobalStore from "@/state/global";
import useSettings from "@/state/settings";
import { useI18n } from "vue-i18n";
import { UnitType } from "@/types/units";

const { t } = useI18n();
const store = useGlobalStore();
const nodes = useNodeStore();
const settings = useSettings();

const form = ref<VForm>();
const tab = ref<number>(0);

const restr = reactive({
  x: false,
  y: false,
  z: false,
  angle: "",
});

const springs = reactive({
  x: "",
  y: "",
  z: "",
});

const displacement = reactive({
  x: "",
  y: "",
  z: "",
});

function onReset() {
  if (!form.value) {
    return;
  }
  form.value.reset();
}

async function onSubmit() {
  if (!form.value) {
    return;
  }

  const { valid } = await form.value.validate();

  if (valid) {
    const rx = restr.x;
    const ry = restr.y;
    const rz = restr.z;
    const angle = parseNumber(restr.angle);

    const kx = rx ? 0 : parseNumber(springs.x);
    const ky = ry ? 0 : parseNumber(springs.y);
    const kz = rz ? 0 : parseNumber(springs.z);

    const ux = rx ? parseNumber(displacement.x) : 0;
    const uy = ry ? parseNumber(displacement.y) : 0;
    const uz = rz ? parseNumber(displacement.z) : 0;

    nodes.applySupports(
      { x: rx, y: ry, z: rz, angle },
      { x: kx, y: ky, z: kz },
      { x: ux, y: uy, z: uz },
    );
  }
}

function validNumber(value: string): boolean | string {
  return isValidNumber(value) || t("errors.validNumber");
}

function nonNegative(value: string): boolean | string {
  return isValidNumber(value, false) || t("errors.nonNegative");
}

function onClose(): void {
  store.current.clickType = ClickType.Select;
  store.current.sideBarType = SideBarType.Select;
}
</script>
