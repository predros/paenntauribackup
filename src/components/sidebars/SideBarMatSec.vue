<template>
  <div>
    <v-card-title class="pb-10">
      {{ t("sidebars.matSec.title") }}
    </v-card-title>

    <v-card-text>
      <v-form
        ref="form"
        validate-on="submit"
      >
        <v-row>
          <v-select
            v-model="store.current.material"
            :label="t('sidebars.matSec.material')"
            :items="materialsList"
            item-title="name"
            item-value="id"
            hide-no-data
            clearable
            prepend-inner-icon="mdi-atom"
          />
        </v-row>

        <v-row>
          <v-select
            v-model="store.current.section"
            :label="t('sidebars.matSec.section')"
            :items="sectionsList"
            item-title="name"
            item-value="id"
            hide-no-data
            clearable
          >
            <template #prepend-inner>
              <IconBase
                :width="30"
                :height="30"
                :icon-color="'#767676'"
              >
                <IconSections />
              </IconBase>
            </template>
          </v-select>
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
          @click="onManageMaterials"
        >
          {{ t("buttons.manageMaterials") }}
        </v-btn>
      </v-row>
      <v-row class="pt-2 px-2">
        <v-btn
          block
          @click="onManageSections"
        >
          {{ t("buttons.manageSections") }}
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

    <v-dialog
      v-model="dialog.material"
      persistent
    >
      <DialogMaterials @close="onCloseDialog" />
    </v-dialog>

    <v-dialog
      v-model="dialog.section"
      persistent
    >
      <DialogSections @close="onCloseDialog" />
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive } from "vue";
import { useI18n } from "vue-i18n";
import useGlobalStore from "@/state/global";
import DialogMaterials from "@/components/dialogs/DialogMaterials.vue";
import IconBase from "@/components/icons/IconBase.vue";
import IconSections from "@/components/icons/IconSections.vue";
import { ClickType, SideBarType } from "@/types/types";
import DialogSections from "../dialogs/DialogSections.vue";

const { t } = useI18n();
const store = useGlobalStore();

const dialog = reactive({
  material: false,
  section: false,
});

const materialsList = computed(() => store.materialsList);
const sectionsList = computed(() => store.sectionsList);

async function onSubmit(): Promise<void> {
  await store.selectedApplyMatSec(
    store.current.material,
    store.current.section,
  );
}

function onManageMaterials(): void {
  dialog.material = true;
  dialog.section = false;
}

function onManageSections(): void {
  dialog.material = false;
  dialog.section = true;
}

function onClose(): void {
  store.current.clickType = ClickType.Select;
  store.current.sideBarType = SideBarType.Select;
}

function onCloseDialog(): void {
  dialog.material = false;
  dialog.section = false;
}
</script>
