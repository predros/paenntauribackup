<template>
  <div>
    <v-card-title class="pb-8">
      {{ t("sidebars.member.title") }}
    </v-card-title>

    <v-card-text>
      <v-form ref="form" validate-on="submit">
        <v-row>
          <v-col>
            <v-text-field
              v-model="member.x0"
              :rules="[validNumber]"
              :label="t('sidebars.member.xStart')"
              :placeholder="`(${settings.getUnitName(UnitType.Length)})`"
              density="comfortable"
              prepend-inner-icon="mdi-axis-arrow"
              @keydown.stop
            />
          </v-col>
          <v-col>
            <v-text-field
              v-model="member.y0"
              :rules="[validNumber]"
              :label="t('sidebars.member.yStart')"
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
              v-model="member.x1"
              :rules="[validNumber]"
              :label="t('sidebars.member.xEnd')"
              :placeholder="`(${settings.getUnitName(UnitType.Length)})`"
              density="comfortable"
              prepend-inner-icon="mdi-axis-arrow"
              @keydown.stop
            />
          </v-col>
          <v-col>
            <v-text-field
              v-model="member.y1"
              :rules="[validNumber]"
              :label="t('sidebars.member.yEnd')"
              :placeholder="`(${settings.getUnitName(UnitType.Length)})`"
              density="comfortable"
              prepend-inner-icon="mdi-axis-arrow"
              @keydown.stop
            />
          </v-col>
        </v-row>

        <v-row>
          <v-select
            v-model="store.current.material"
            :label="t('sidebars.member.material')"
            :items="materialsList"
            :rules="[validMaterial]"
            item-title="name"
            item-value="id"
            density="comfortable"
            hide-no-data
            prepend-inner-icon="mdi-atom"
          />
        </v-row>

        <v-row>
          <v-select
            v-model="store.current.section"
            :label="t('sidebars.member.section')"
            :items="sectionsList"
            :rules="[validSection, distinctPoints, uniqueMember]"
            item-title="name"
            item-value="id"
            density="comfortable"
            hide-no-data
          >
            <template #prepend-inner>
              <IconBase :width="30" :height="30" :icon-color="'#767676'">
                <IconSections />
              </IconBase>
            </template>
          </v-select>
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
import { ref, reactive, computed } from "vue";
import { VForm } from "vuetify/components";
import { floatEq } from "@/helper/math";
import { isValidNumber, parseNumber } from "@/helper/misc";

import IconBase from "@/components/icons/IconBase.vue";
import IconSections from "@/components/icons/IconSections.vue";
import { IMaterial, ISection, ClickType, SideBarType } from "@/types/types";

import useGlobalStore from "@/state/global";
import useMemberStore from "@/state/members";
import useSettings from "@/state/settings";
import { useI18n } from "vue-i18n";
import { UnitType } from "@/types/units";

const { t } = useI18n();
const store = useGlobalStore();
const members = useMemberStore();
const settings = useSettings();

const form = ref<VForm>();

const materialsList = computed<IMaterial[]>(() => store.materialsList);
const sectionsList = computed<ISection[]>(() => store.sectionsList);

const member = reactive({
  x0: "",
  y0: "",
  x1: "",
  y1: "",
});

function validNumber(value: string): boolean | string {
  return isValidNumber(value, true, true) || t("errors.validNumber");
}

function validMaterial(value: number): boolean | string {
  return value != null || t("errors.validMaterial");
}

function validSection(value: number): boolean | string {
  return value != null || t("errors.validSection");
}

function distinctPoints(): boolean | string {
  const parsedX0 = parseNumber(member.x0);
  const parsedY0 = parseNumber(member.y0);
  const parsedX1 = parseNumber(member.x1);
  const parsedY1 = parseNumber(member.y1);

  if (
    Number.isNaN(parsedX0) ||
    Number.isNaN(parsedX1) ||
    Number.isNaN(parsedY0) ||
    Number.isNaN(parsedY1)
  )
    return true; // The other rules will deal with this case

  return (
    (parsedX0 != parsedX1 && parsedY0 != parsedY1) || t("errors.distinctPoints")
  );
}

function uniqueMember(): boolean | string {
  const parsedX0 = parseNumber(member.x0);
  const parsedY0 = parseNumber(member.y0);
  const parsedX1 = parseNumber(member.x1);
  const parsedY1 = parseNumber(member.y1);

  if (
    Number.isNaN(parsedX0) ||
    Number.isNaN(parsedX1) ||
    Number.isNaN(parsedY0) ||
    Number.isNaN(parsedY1)
  )
    return true; // The other rules will deal with this case

  let exists = false;

  members.membersList.forEach((member) => {
    if (
      floatEq(member.x0, parsedX0) &&
      floatEq(member.y0, parsedY0) &&
      floatEq(member.x1, parsedX1) &&
      floatEq(member.y1, parsedY1)
    ) {
      exists = true;
      return;
    }
    if (
      floatEq(member.x1, parsedX0) &&
      floatEq(member.y1, parsedY0) &&
      floatEq(member.x0, parsedX1) &&
      floatEq(member.y0, parsedY1)
    ) {
      exists = true;
      return;
    }
  });

  return exists == false || t("errors.uniqueMember");
}

async function onSubmit(): Promise<void> {
  if (!form.value) return;

  const { valid } = await form.value.validate();

  if (valid) {
    const parsedX0 = parseNumber(member.x0);
    const parsedY0 = parseNumber(member.y0);
    const parsedX1 = parseNumber(member.x1);
    const parsedY1 = parseNumber(member.y1);

    if (store.current.material == null || store.current.section == null) return;

    members.newMember(
      parsedX0,
      parsedY0,
      parsedX1,
      parsedY1,
      store.current.material,
      store.current.section,
    );

    const deltaX = parsedX1 - parsedX0;
    const deltaY = parsedY1 - parsedY0;

    member.x0 = String(parsedX1);
    member.y0 = String(parsedX1);
    member.x1 = String(parsedX1 + deltaX);
    member.y1 = String(parsedY1 + deltaY);
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
