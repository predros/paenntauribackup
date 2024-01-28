<template>
  <div>
    <v-btn
      id="menu-file"
      class="text-none"
    >
      {{ t("menu.file.self") }}
    </v-btn>

    <v-btn
      id="menu-edit"
      class="text-none"
    >
      {{ t("menu.edit.self") }}
    </v-btn>
  </div>

  <div>
    <v-menu
      activator="#menu-file"
      location="bottom"
    >
      <v-list>
        <v-list-item
          density="compact"
          @click="async () => await store.fileNew()"
        >
          <template #prepend>
            <v-icon>mdi-file</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.file.new") }}</div>
          </template>
          <template #append> Ctrl+N </template>
        </v-list-item>

        <v-list-item
          density="compact"
          @click="async () => await store.fileOpen()"
        >
          <template #prepend>
            <v-icon>mdi-folder-open</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.file.open") }}</div>
          </template>
          <template #append> Ctrl+O </template>
        </v-list-item>

        <v-list-item
          density="compact"
          @click="async () => await store.fileSave()"
        >
          <template #prepend>
            <v-icon>mdi-content-save</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.file.save") }}</div>
          </template>
          <template #append> Ctrl+S </template>
        </v-list-item>

        <v-list-item
          density="compact"
          @click="async () => await store.fileSaveAs()"
        >
          <template #prepend>
            <v-icon>mdi-content-save-move</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.file.saveAs") }}</div>
          </template>
          <template #append> Ctrl+Shift+S </template>
        </v-list-item>

        <v-divider />

        <v-list-item
          density="compact"
          @click="async () => await store.appExit()"
        >
          <template #prepend>
            <v-icon>mdi-exit-to-app</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.file.exit") }}</div>
          </template>
          <template #append> Ctrl+W </template>
        </v-list-item>
      </v-list>
    </v-menu>

    <v-menu
      activator="#menu-edit"
      location="bottom"
    >
      <v-list>
        <v-list-item
          :disabled="
            store.historyLength.undo == 0 ||
            store.current.sideBarType == SideBarType.Result
          "
          density="compact"
          @click="() => store.appUndo()"
        >
          <template #prepend>
            <v-icon>mdi-undo</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.edit.undo") }}</div>
          </template>
          <template #append> Ctrl+Z </template>
        </v-list-item>

        <v-list-item
          :disabled="
            store.historyLength.redo == 0 ||
            store.current.sideBarType == SideBarType.Result
          "
          density="compact"
          @click="() => store.appRedo()"
        >
          <template #prepend>
            <v-icon>mdi-redo</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.edit.redo") }}</div>
          </template>
          <template #append> Ctrl+Y </template>
        </v-list-item>

        <v-list-item
          :disabled="store.current.sideBarType == SideBarType.Result"
          density="compact"
          @click="() => store.selectedDelete()"
        >
          <template #prepend>
            <v-icon>mdi-trash-can</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.edit.selectedDelete") }}</div>
          </template>
          <template #append> Del </template>
        </v-list-item>

        <v-divider />

        <v-list-item
          :disabled="store.current.sideBarType == SideBarType.Result"
          density="compact"
          @click="() => (settings.showSettingsDialog = true)"
        >
          <template #prepend>
            <v-icon>mdi-cog</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.edit.settings") }}</div>
          </template>
        </v-list-item>
      </v-list>
    </v-menu>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import useGlobalStore from "@/state/global";
import useSettings from "@/state/settings";
import { SideBarType } from "@/types/types";

const { t } = useI18n();
const store = useGlobalStore();
const settings = useSettings();
</script>
