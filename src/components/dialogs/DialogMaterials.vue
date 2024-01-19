<template>
  <v-card class="py-4 px-6" style="min-width: 600px; min-height: 500px">
    <v-card-title class="pb-6">
      {{ t("dialogs.materials.title") }}
    </v-card-title>

    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="store.materialsList"
        items-per-page="10"
        :sort-by="[{ key: 'name', order: 'asc' }]"
        item-key="id"
        height="380"
      >
        <template #item="{ item }">
          <tr>
            <td>{{ item.name }}</td>
            <td>
              {{
                settings.formatUnit(item.elasticity, UnitType.Elasticity, false)
              }}
            </td>
            <td>
              {{ settings.formatUnit(item.thermal, UnitType.Thermal, false) }}
            </td>
            <td>
              <v-btn icon elevation="0" @click="onShowForm(item)">
                <v-icon>mdi-pencil</v-icon>
                <v-tooltip location="bottom" activator="parent">
                  {{ t("buttons.edit") }}
                </v-tooltip>
              </v-btn>
              <v-btn icon elevation="0" @click="onShowDelete(item)">
                <v-icon>mdi-trash-can</v-icon>
                <v-tooltip location="bottom" activator="parent">
                  {{ t("buttons.delete") }}
                </v-tooltip>
              </v-btn>
            </td>
          </tr>
        </template>
      </v-data-table>
    </v-card-text>

    <div class="d-flex justify-end">
      <v-btn
        class="pl-3 pr-3 mr-3"
        color="primary"
        prepend-icon="mdi-plus"
        @click="() => onShowForm(null)"
      >
        {{ t("buttons.newMaterial") }}
      </v-btn>

      <v-btn class="pl-3 pr-3" @click="onClose">
        {{ t("buttons.close") }}
      </v-btn>
    </div>
  </v-card>

  <v-dialog v-model="dialogs.form" persistent width="400">
    <v-card class="py-4 px-6">
      <v-card-title class="pb-7">
        {{
          currentMaterial.id == null
            ? t("dialogs.materials.newMaterial")
            : t("dialogs.materials.editing", [currentMaterial.name])
        }}
      </v-card-title>

      <v-card-text>
        <v-form ref="form" validate-on="submit">
          <v-row>
            <v-text-field
              v-model="formMaterial.name"
              :rules="[validName]"
              :label="t('dialogs.materials.name')"
              prepend-inner-icon="mdi-tag"
            />
          </v-row>

          <v-row>
            <v-text-field
              v-model="formMaterial.elasticity"
              :rules="[validNumber, isPositive]"
              :label="t('dialogs.materials.elasticity')"
              :placeholder="`(${settings.getUnitName(UnitType.Elasticity)})`"
              prepend-inner-icon="mdi-atom"
            />
          </v-row>

          <v-row>
            <v-text-field
              v-model="formMaterial.thermal"
              :rules="[validNumber, isPositive]"
              :label="t('dialogs.materials.thermal')"
              :placeholder="`(${settings.getUnitName(UnitType.Thermal)})`"
              prepend-inner-icon="mdi-thermometer"
            />
          </v-row>
        </v-form>
      </v-card-text>

      <div class="d-flex justify-end">
        <v-btn class="pl-3 pr-3 mr-3" color="primary" @click="onSubmitForm">
          {{ t("buttons.save") }}
        </v-btn>

        <v-btn class="pl-3 pr-3" @click="onDialogClose">
          {{ t("buttons.cancel") }}
        </v-btn>
      </div>
    </v-card>
  </v-dialog>

  <v-dialog v-model="dialogs.delete" persistent>
    <v-card class="py-4 px-6">
      <v-card-title>
        {{
          currentMaterial == null
            ? t("errors.ERROR")
            : t("dialogs.materials.deleting", [currentMaterial.name])
        }}
      </v-card-title>

      <v-card-text>
        {{ t("dialogs.materials.areYouSure", [currentMaterial.name]) }}
      </v-card-text>

      <div class="d-flex justify-end">
        <v-btn
          v-if="currentMaterial.id != null"
          class="pl-3 pr-3 mr-3"
          color="primary"
          @click="onSubmitDelete"
        >
          {{ t("buttons.delete") }}
        </v-btn>

        <v-btn class="pl-3 pr-3" @click="onDialogClose">
          {{ t("buttons.cancel") }}
        </v-btn>
      </div>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, defineEmits } from "vue";
import { IMaterial } from "@/types/types";
import { isValidNumber, parseNumber } from "@/helper/misc";
import { VForm } from "vuetify/components";
import { useI18n } from "vue-i18n";

import useGlobalStore from "@/state/global";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";
import { UnitType } from "@/types/units";

const { t } = useI18n();
const emit = defineEmits(["close"]);

const store = useGlobalStore();
const members = useMemberStore();
const settings = useSettings();

const form = ref<VForm>();

const headers = reactive([
  {
    title: t("dialogs.materials.name"),
    value: "name",
    width: "40%",
    sortable: true,
  },
  {
    title:
      t("dialogs.materials.elasticity") +
      ` (${settings.getUnitName(UnitType.Elasticity)})`,
    value: "elasticity",
    width: "20%",
  },
  {
    title:
      t("dialogs.materials.thermal") +
      ` (${settings.getUnitName(UnitType.Thermal)})`,
    value: "thermal",
    width: "20%",
  },
  {
    title: "",
    value: "",
    width: "20%",
  },
]);

const currentMaterial = reactive({
  id: null as number | null,
  name: "",
});

const formMaterial = reactive({
  name: "",
  elasticity: "",
  thermal: "",
});

const dialogs = reactive({
  form: false,
  delete: false,
});

function onClose(): void {
  emit("close");
}

function onShowForm(material: IMaterial | null) {
  if (material == null) {
    currentMaterial.id = null;
    currentMaterial.name = "";

    formMaterial.name = "";
    formMaterial.elasticity = "";
    formMaterial.thermal = "";
  } else {
    currentMaterial.id = material.id;
    currentMaterial.name = material.name;

    formMaterial.name = material.name;
    formMaterial.elasticity = String(material.elasticity);
    formMaterial.thermal = String(material.thermal);
  }
  dialogs.form = true;
  dialogs.delete = false;
}

function onShowDelete(material: IMaterial) {
  if (store.materialsList.length < 2) {
    store.showAlert("alerts.singleMaterial");
    return;
  }

  const inUse = members.membersList.some((m) => m.material == material.id);
  if (inUse) {
    store.showAlert("alerts.materialInUse");
    return;
  }

  currentMaterial.id = material.id;
  currentMaterial.name = material.name;

  dialogs.form = false;
  dialogs.delete = true;
}

async function onSubmitForm() {
  if (!form.value) return;

  const { valid } = await form.value.validate();

  if (valid) {
    const sameName = store.materialsList.find(
      (x) => x.name == formMaterial.name,
    );

    if (sameName != undefined && sameName.id != currentMaterial.id) {
      store.showAlert(t("alerts.materialNameInUse"));
      return;
    }

    const elasticity = parseNumber(formMaterial.elasticity);
    const thermal = parseNumber(formMaterial.thermal);

    if (currentMaterial.id == null) {
      await store.newMaterial(formMaterial.name, elasticity, thermal);
    } else {
      await store.updateMaterial(
        currentMaterial.id,
        formMaterial.name,
        elasticity,
        thermal,
      );
    }
    onDialogClose();
  }
}

async function onSubmitDelete() {
  if (currentMaterial.id == null) return;
  await store.deleteMaterial(currentMaterial.id);
  onDialogClose();
}

function onDialogClose() {
  currentMaterial.id = null;
  dialogs.form = false;
  dialogs.delete = false;
}

function validName(value: string): boolean | string {
  return !(value.trim() == "") || t("errors.validName");
}

function validNumber(value: string): boolean | string {
  return isValidNumber(value, true, true) || t("errors.validNumber");
}

function isPositive(value: string): boolean | string {
  return isValidNumber(value, false, false) || t("errors.isPositive");
}
</script>
