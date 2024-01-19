<template>
  <v-card class="py-4 px-6" style="min-width: 600px; min-height: 500px">
    <v-card-title class="pb-6">
      {{ t("dialogs.sections.title") }}
    </v-card-title>

    <v-card-text>
      <v-data-table
        :headers="[
          {
            title: t('dialogs.sections.name'),
            value: 'name',
            width: '32%',
            sortable: true,
          },
          {
            title: t('dialogs.sections.type'),
            value: '',
            width: '17%',
          },
          {
            title:
              t('dialogs.sections.inertia') +
              ` (${settings.getUnitName(UnitType.Inertia)})`,
            value: 'inertia',
            width: '17%',
          },
          {
            title:
              t('dialogs.sections.area') +
              ` (${settings.getUnitName(UnitType.Area)})`,
            value: 'area',
            width: '17%',
          },
          {
            title: '',
            value: '',
            width: '17%',
          },
        ]"
        :items="store.sectionsList"
        items-per-page="10"
        :sort-by="[{ key: 'name', order: 'asc' }]"
        item-key="id"
        height="380"
      >
        <template #item="{ item }">
          <tr>
            <td>{{ item.name }}</td>
            <td>{{ getSectionTypeName(item.section_type) }}</td>
            <td>
              {{ settings.formatUnit(item.inertia, UnitType.Inertia, false) }}
            </td>
            <td>{{ settings.formatUnit(item.area, UnitType.Area, false) }}</td>
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
        {{ t("buttons.newSection") }}
      </v-btn>

      <v-btn class="pl-3 pr-3" @click="onClose"> Fechar </v-btn>
    </div>
  </v-card>

  <v-dialog v-model="dialogs.form" persistent width="620">
    <v-card class="py-4 px-6" height="650">
      <v-card-title class="pb-7">
        {{
          currentSection.id == null
            ? t("dialogs.sections.newSection")
            : t("dialogs.sections.editing", [currentSection.name])
        }}
      </v-card-title>

      <v-card-text
        class="text-center d-flex flex-column align-space-between justify-center"
      >
        <v-row align="center" justify="center">
          <v-col
            class="text-center flex-column align-center justify-center"
            height="100%"
          >
            <v-form ref="form" validate-on="submit">
              <v-row>
                <v-col>
                  <v-text-field
                    v-model="formSection.name"
                    :rules="[validName]"
                    :label="t('dialogs.sections.name')"
                    prepend-inner-icon="mdi-tag"
                  />
                </v-col>
              </v-row>

              <v-row>
                <v-col>
                  <v-select
                    v-model="formSection.sectionType"
                    :label="t('dialogs.sections.type')"
                    :items="[
                      {
                        title: t('dialogs.sections.types.generic'),
                        value: SectionType.Generic,
                      },
                      {
                        title: t('dialogs.sections.types.circle'),
                        value: SectionType.Circle,
                      },
                      {
                        title: t('dialogs.sections.types.rectangle'),
                        value: SectionType.Rectangle,
                      },
                    ]"
                  />
                </v-col>
              </v-row>

              <div v-if="formSection.sectionType == SectionType.Generic">
                <v-row>
                  <v-col>
                    <v-text-field
                      v-model="formSection.genericInertia"
                      :rules="[validNumber, isPositive]"
                      :label="t('dialogs.sections.inertia')"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Inertia,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                  <v-col>
                    <v-text-field
                      v-model="formSection.genericArea"
                      :rules="[validNumber, isPositive]"
                      :label="t('dialogs.sections.area')"
                      :placeholder="`(${settings.getUnitName(UnitType.Area)})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                </v-row>

                <v-row>
                  <v-col>
                    <v-text-field
                      v-model="formSection.genericYSup"
                      :rules="[validNumber, isPositive]"
                      label="Ysup"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>

                  <v-col>
                    <v-text-field
                      v-model="formSection.genericYInf"
                      :rules="[validNumber, isPositive]"
                      label="Yinf"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                </v-row>
              </div>

              <div v-else-if="formSection.sectionType == SectionType.Circle">
                <v-row>
                  <v-col>
                    <v-text-field
                      v-model="formSection.circleD"
                      :rules="[validNumber, isPositive]"
                      label="D"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                </v-row>

                <v-row>
                  <v-col>
                    <v-text-field
                      v-model="formSection.circled"
                      :rules="[validNumber, nonNegative, circleInnerDiameter]"
                      label="d"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                </v-row>
              </div>

              <div v-else-if="formSection.sectionType == SectionType.Rectangle">
                <v-row>
                  <v-col>
                    <v-text-field
                      v-model="formSection.rectB"
                      :rules="[validNumber, isPositive]"
                      label="B"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                  <v-col>
                    <v-text-field
                      v-model="formSection.rectH"
                      :rules="[validNumber, isPositive]"
                      label="H"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                </v-row>
                <v-row>
                  <v-col>
                    <v-text-field
                      v-model="formSection.rectb"
                      :rules="[validNumber, nonNegative, rectInnerWidth]"
                      label="b"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                  <v-col>
                    <v-text-field
                      v-model="formSection.recth"
                      :rules="[validNumber, nonNegative, rectInnerHeight]"
                      label="h"
                      :placeholder="`(${settings.getUnitName(
                        UnitType.Dimension,
                      )})`"
                      prepend-inner-icon="mdi-ruler"
                    />
                  </v-col>
                </v-row>
              </div>
            </v-form>
          </v-col>

          <v-col class="d-flex justify-end">
            <v-img
              :width="200"
              :src="`assets/images/sections/${formImg}.svg`"
            />
          </v-col>
        </v-row>
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
          currentSection.id == null
            ? t("errors.ERROR")
            : t("dialogs.sections.deleting", [currentSection.name])
        }}
      </v-card-title>

      <v-card-text>
        {{ t("dialogs.sections.areYouSure", [currentSection.name]) }}
      </v-card-text>

      <div class="d-flex justify-end">
        <v-btn
          v-if="currentSection.id != null"
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
import { ref, reactive, defineEmits, computed } from "vue";

import useGlobalStore from "@/state/global";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";

import { ISection, SectionType } from "@/types/types";
import { isValidNumber, parseNumber } from "@/helper/misc";
import { VForm } from "vuetify/components";
import { UnitType } from "@/types/units";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const emit = defineEmits(["close"]);
const store = useGlobalStore();
const members = useMemberStore();
const settings = useSettings();

const form = ref<VForm>();

const currentSection = reactive({
  id: null as number | null,
  name: "",
});

const formSection = reactive({
  name: "",
  sectionType: SectionType.Generic,

  genericInertia: "",
  genericArea: "",
  genericYSup: "",
  genericYInf: "",

  circleD: "",
  circled: "",

  rectB: "",
  rectH: "",
  rectb: "",
  recth: "",
});

const formImg = computed(() => {
  switch (formSection.sectionType) {
    case SectionType.Circle:
      return "circle";
    case SectionType.Rectangle:
      return "rectangle";
    default:
      return "generic";
  }
});

const dialogs = reactive({
  form: false,
  delete: false,
});

const sectionsList = computed(() => {
  const result = store.sectionsList.slice();
  result.sort((x, y) => {
    const xName = x.name.toLowerCase();
    const yName = y.name.toLowerCase();
    if (xName > yName) return 1;
    if (xName == yName) return 0;
    return -1;
  });
  return result;
});

function getSectionTypeName(sectionType: unknown): string {
  const cast = sectionType as SectionType;

  switch (cast) {
    case SectionType.AsymmetricW:
      return t("dialogs.sections.types.asymmetricW");
    case SectionType.Circle:
      return t("dialogs.sections.types.circle");
    case SectionType.Generic:
      return t("dialogs.sections.types.generic");
    case SectionType.Rectangle:
      return t("dialogs.sections.types.rectangle");
    case SectionType.SymmetricW:
      return t("dialogs.sections.types.symmetricW");
    default:
      return t("errors.ERROR");
  }
}

function clearForm(): void {
  formSection.name = "";
  formSection.sectionType = SectionType.Generic;
  formSection.circleD = "";
  formSection.circled = "";
  formSection.genericArea = "";
  formSection.genericInertia = "";
  formSection.genericYInf = "";
  formSection.genericYSup = "";
  formSection.rectB = "";
  formSection.rectH = "";
  formSection.rectb = "";
  formSection.recth = "";
}

function onClose(): void {
  emit("close");
}

function onShowForm(section: ISection | null) {
  clearForm();
  if (section == null) {
    currentSection.id = null;
    currentSection.name = "";
  } else {
    currentSection.id = section.id;
    currentSection.name = section.name;

    formSection.name = section.name;
    formSection.sectionType = section.section_type;

    switch (section.section_type) {
      case SectionType.Generic:
        formSection.genericInertia = String(section.inertia);
        formSection.genericArea = String(section.area);
        formSection.genericYSup = String(section.y_sup);
        formSection.genericYInf = String(section.y_inf);
        break;
      case SectionType.Circle:
        formSection.circleD = String(section.params[0]);
        formSection.circled = String(section.params[1]);
        break;
      case SectionType.Rectangle:
        formSection.rectB = String(section.params[0]);
        formSection.rectH = String(section.params[1]);
        formSection.rectb = String(section.params[2]);
        formSection.recth = String(section.params[3]);
        break;
    }
  }
  dialogs.form = true;
  dialogs.delete = false;
}

function onShowDelete(section: ISection) {
  if (store.sectionsList.length < 2) {
    store.showAlert("alerts.singleSection");
    return;
  }

  const inUse = members.membersList.some((m) => m.section == section.id);
  if (inUse) {
    store.showAlert("alerts.sectionInUse");
    return;
  }

  currentSection.id = section.id;
  currentSection.name = section.name;

  dialogs.form = false;
  dialogs.delete = true;
}

async function onSubmitForm() {
  if (!form.value) return;

  const { valid } = await form.value.validate();

  if (valid) {
    const sameName = sectionsList.value.find((x) => x.name == formSection.name);

    if (sameName != undefined && sameName.id != currentSection.id) {
      store.showAlert(t("alerts.sectionNameInUse"));
      return;
    }

    const params = [] as number[];
    switch (formSection.sectionType) {
      case SectionType.Circle:
        params.push(Number(formSection.circleD));
        params.push(Number(formSection.circled));
        break;
      case SectionType.Generic:
        params.push(Number(formSection.genericInertia));
        params.push(Number(formSection.genericArea));
        params.push(Number(formSection.genericYSup));
        params.push(Number(formSection.genericYInf));
        break;
      case SectionType.Rectangle:
        params.push(Number(formSection.rectB));
        params.push(Number(formSection.rectH));
        params.push(Number(formSection.rectb));
        params.push(Number(formSection.recth));
        break;
    }

    if (currentSection.id == null) {
      store.newSection(formSection.name, formSection.sectionType, params);
    } else {
      store.updateSection(
        currentSection.id,
        formSection.name,
        formSection.sectionType,
        params,
      );
    }

    clearForm();
    onDialogClose();
  }
}

function onSubmitDelete() {
  if (currentSection.id == null) return;
  store.deleteSection(currentSection.id);
  onDialogClose();
}

function onDialogClose() {
  currentSection.id = null;
  dialogs.form = false;
  dialogs.delete = false;
}

function validName(value: string): boolean | string {
  return !(value.trim() == "") || t("errors.validName");
}

function validNumber(value: string): boolean | string {
  return isValidNumber(value) || t("errors.validNumber");
}

function nonNegative(value: string): boolean | string {
  return isValidNumber(value, false, true) || t("errors.nonNegative");
}

function isPositive(value: string): boolean | string {
  return isValidNumber(value, false, false) || t("errors.isPositive");
}

function circleInnerDiameter(): boolean | string {
  return (
    parseNumber(formSection.circleD) > parseNumber(formSection.circled) ||
    t("errors.innerDimensionLarger")
  );
}

function rectInnerWidth(): boolean | string {
  return (
    parseNumber(formSection.rectB) > parseNumber(formSection.rectb) ||
    t("errors.innerDimensionLarger")
  );
}

function rectInnerHeight(): boolean | string {
  return (
    parseNumber(formSection.rectH) > parseNumber(formSection.recth) ||
    t("errors.innerDimensionLarger")
  );
}
</script>
