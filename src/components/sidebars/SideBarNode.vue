<template>
  <div>
    <v-card-title class="pb-10">
      {{ t("sidebars.node.title") }}
    </v-card-title>

    <v-card-text>
      <v-form ref="form" validate-on="submit">
        <v-row>
          <v-col>
            <v-text-field
              v-model="node.x"
              :rules="[validNumber, uniqueNode]"
              :label="t('sidebars.node.x')"
              :placeholder="`(${settings.getUnitName(UnitType.Length)})`"
              density="comfortable"
              prepend-inner-icon="mdi-axis-arrow"
              @keydown.stop
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col>
            <v-text-field
              v-model="node.y"
              :rules="[validNumber, uniqueNode]"
              :label="t('sidebars.node.y')"
              :placeholder="`(${settings.getUnitName(UnitType.Length)})`"
              density="comfortable"
              prepend-inner-icon="mdi-axis-arrow"
              @keydown.stop
            />
          </v-col>
        </v-row>
      </v-form>

      <v-row class="pt-10 px-2">
        <v-btn block color="primary" @click="onSubmit">
          {{ t("buttons.save") }}
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
import { VForm } from "vuetify/components";
import { floatEq } from "@/helper/math";
import { isValidNumber, parseNumber } from "@/helper/misc";
import useNodeStore from "@/state/nodes";
import useGlobalStore from "@/state/global";
import useSettings from "@/state/settings";
import { ClickType, SideBarType } from "@/types/types";
import { useI18n } from "vue-i18n";
import { UnitType } from "@/types/units";

const { t } = useI18n();
const store = useGlobalStore();
const nodes = useNodeStore();
const settings = useSettings();

const form = ref<VForm>();

const node = reactive({
  x: "",
  y: "",
});

function validNumber(value: string): boolean | string {
  return isValidNumber(value, true, true) || t("errors.validNumber");
}

function uniqueNode(): boolean | string {
  const parsedX = parseNumber(node.x);
  const parsedY = parseNumber(node.y);

  if (Number.isNaN(parsedX) || Number.isNaN(parsedY)) return true;

  let exists = false;
  nodes.nodesList.forEach((n) => {
    if (floatEq(n.x, parsedX) && floatEq(n.y, parsedY)) {
      exists = true;
      return;
    }
  });

  return !exists || t("errors.uniqueNode");
}

async function onSubmit(): Promise<void> {
  if (!form.value) return;

  const { valid } = await form.value.validate();

  if (valid) {
    const xParsed = parseNumber(node.x);
    const yParsed = parseNumber(node.y);

    nodes.newNode(xParsed, yParsed);
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
