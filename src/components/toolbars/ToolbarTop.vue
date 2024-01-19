<template>
  <div class="toolbar">
    <v-btn-group rounded="0">
      <v-btn @click="async () => await store.newFile()">
        <v-icon size="28">mdi-file</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.new") }}
        </v-tooltip>
      </v-btn>

      <v-btn @click="async () => await store.openFile()">
        <v-icon size="28">mdi-folder-open</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.open") }}
        </v-tooltip>
      </v-btn>

      <v-btn @click="async () => await store.saveFile()">
        <v-icon size="28">mdi-content-save</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.save") }}
        </v-tooltip>
      </v-btn>

      <v-btn @click="async () => await store.saveFileAs()">
        <v-icon size="28">mdi-content-save-move</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.saveAs") }}
        </v-tooltip>
      </v-btn>

      <v-btn
        :disabled="
          store.historyLength.undo == 0 ||
          store.current.sideBarType == SideBarType.Result
        "
        @click="() => store.undo()"
      >
        <v-icon size="28">mdi-undo</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.undo") }}
        </v-tooltip>
      </v-btn>

      <v-btn
        :disabled="
          store.historyLength.redo == 0 ||
          store.current.sideBarType == SideBarType.Result
        "
        @click="() => store.redo()"
      >
        <v-icon size="28">mdi-redo</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.redo") }}
        </v-tooltip>
      </v-btn>

      <v-btn @click="() => (settings.showSettingsDialog = true)">
        <v-icon size="28">mdi-cog</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.settings") }}
        </v-tooltip>
      </v-btn>

      <v-btn @click="() => store.showDialog.loadcases = true" >
        <IconBase :width="35" :height="35" icon-name="loads">
          <IconLoads />
        </IconBase>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.loadcases") }}
        </v-tooltip>
      </v-btn>

      <v-btn @click="() => store.runAnalysisLinear()">
        <v-icon size="28">mdi-play</v-icon>
        <v-tooltip activator="parent" location="bottom">
          {{ t("toolbars.top.run") }}
        </v-tooltip>
      </v-btn>
    </v-btn-group>

    <v-spacer />

    <v-select
      v-model="loadcase"
      :label="t('toolbars.top.loadcaseCurrent')"
      :items="store.loadcasesList"
      :disabled="store.current.sideBarType == SideBarType.Result"
      variant="solo"
      item-title="name"
      item-value="id"
      density="comfortable"
      hide-no-data
      style="max-width: 300px"
      @update:model-value="onCaseChange"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import useGlobalStore from "@/state/global";
import useSettings from "@/state/settings";

import IconBase from "@/components/icons/IconBase.vue";
import IconLoads from "@/components/icons/IconLoads.vue";
import { useI18n } from "vue-i18n";
import { SideBarType } from "@/types/types";

const { t } = useI18n();
const store = useGlobalStore();
const settings = useSettings();

const loadcase = ref<number>(0);

function onCaseChange(): void {
  store.changeCurrentLoadcase(loadcase.value);
}
</script>

<style scoped>
.toolbar {
  display: flex;
  flex-flow: row;
  flex: 1;
}
</style>
