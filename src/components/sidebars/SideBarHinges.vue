<template>
  <div>
    <v-card-title class="pb-8">
      {{ t("sidebars.hinges.title") }}
    </v-card-title>

    <v-card-text>
      <v-row>
        <v-checkbox
          v-model="hinges.nodes"
          density="comfortable"
          :label="t('sidebars.hinges.nodes')"
        />
      </v-row>
      <v-row>
        <v-checkbox
          v-model="hinges.memberStart"
          density="comfortable"
          :label="t('sidebars.hinges.memberStarts')"
        />
      </v-row>
      <v-row>
        <v-checkbox
          v-model="hinges.memberEnd"
          density="comfortable"
          :label="t('sidebars.hinges.memberEnds')"
        />
      </v-row>

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
import { reactive } from "vue";
import { useI18n } from "vue-i18n";
import { ClickType, SideBarType } from "@/types/types";
import useGlobalStore from "@/state/global";

const { t } = useI18n();
const store = useGlobalStore();

const hinges = reactive({
  nodes: false,
  memberStart: false,
  memberEnd: false,
});

function onReset(): void {
  hinges.nodes = false;
  hinges.memberStart = false;
  hinges.memberEnd = false;
}

function onSubmit(): void {
  store.selectedApplyHinges(hinges.nodes, hinges.memberStart, hinges.memberEnd);
}

function onClose(): void {
  store.current.clickType = ClickType.Select;
  store.current.sideBarType = SideBarType.Select;
}
</script>
