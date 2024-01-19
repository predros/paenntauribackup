<template>
  <div>
    <v-btn id="menu-file" class="text-none">
      {{ t("menu.file.self") }}
    </v-btn>

    <v-btn id="menu-edit" class="text-none">
      {{ t("menu.edit.self") }}
    </v-btn>

    <v-btn id="menu-structure" class="text-none" disabled>
      {{ t("menu.structure.self") }}
    </v-btn>

    <v-btn id="menu-analysis" class="text-none" disabled>
      {{ t("menu.analysis.self") }}
    </v-btn>

    <v-btn id="menu-help" class="text-none" disabled>
      {{ t("menu.help.self") }}
    </v-btn>
  </div>

  <div>
    <v-menu activator="#menu-file" location="bottom">
      <v-list>
        <v-list-item
          density="compact"
          @click="async () => await store.newFile()"
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
          @click="async () => await store.openFile()"
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
          @click="async () => await store.saveFile()"
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
          @click="async () => await store.saveFileAs()"
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
          @click="async () => await store.exitApp()"
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

    <v-menu activator="#menu-edit" location="bottom">
      <v-list>
        <v-list-item
          :disabled="
            store.historyLength.undo == 0 ||
            store.current.sideBarType == SideBarType.Result
          "
          density="compact"
          @click="() => store.undo()"
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
          @click="() => store.redo()"
        >
          <template #prepend>
            <v-icon>mdi-redo</v-icon>
          </template>
          <template #default>
            <div class="pr-3">{{ t("menu.edit.redo") }}</div>
          </template>
          <template #append> Ctrl+Y </template>
        </v-list-item>
      </v-list>
    </v-menu>

    <v-menu activator="#menu-structure" location="bottom"> </v-menu>

    <v-menu activator="#menu-analysis" location="bottom"> </v-menu>

    <v-menu activator="#menu-help" location="bottom"> </v-menu>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import useGlobalStore from "@/state/global";
import { SideBarType } from "@/types/types";

const { t } = useI18n();
const store = useGlobalStore();
</script>
