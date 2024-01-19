<template>
<v-card class="py-4 px-6" style="min-width: 600px; min-height: 500px">
  <v-card-title class="pb-6">
    Casos de carga/Combinações
  </v-card-title>

  <v-card-text>
    <v-tabs v-model="tab" background-color="primary">
      <v-tab :value="0">Casos de carga</v-tab>
      <v-tab :value="1">Combinações</v-tab>
    </v-tabs>

    <v-window v-model="tab" class="pt-3">
      <v-window-item :value="0">
        <v-data-table
          :headers="[
            {
              title: 'Nome',
              value: 'name',
              width: '70%',
              sortable: true,
            },
            {
              title: '',
              value: '',
              width: '30%',
              sortable: false,
            },
          ]"
          :items="store.loadcasesList"
          items-per-page="10"
          :sort-by="[{ key: 'name', order: 'asc' }]"
          item-key="id"
          height="380"
        >
          <template #item="{ item }">
            <tr>
              <td>{{ item.name }}</td>
              <td>
                <v-btn icon elevation="0" @click="onShowLoadcaseForm(item)">
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
      </v-window-item>

      <v-window-item :value="1">
        <v-data-table
          :headers="[
            {
              title: 'Nome',
              value: 'name',
              width: '70%',
              sortable: true,
            },
            {
              title: '',
              value: '',
              width: '30%',
              sortable: false,
            },
          ]"
          :items="store.combinationsList"
          items-per-page="10"
          :sort-by="[{ key: 'name', order: 'asc' }]"
          item-key="id"
          height="380"
        >
          <template #item="{ item }">
            <tr>
              <td>{{ item.name }}</td>
              <td>
                <v-btn icon elevation="0" @click="onShowLoadcaseForm(item)">
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
      </v-window-item>
    </v-window>
  </v-card-text>

  <div class="d-flex justify-end">
    <v-btn
      v-if="tab == 0"
      class="pl-3 pr-3 mr-3"
      color="primary"
      prepend-icon="mdi-plus"
      @click="() => onShowLoadcaseForm(null)"
    >
      Novo caso de carga
    </v-btn>
    <v-btn
      v-else
      class="pl-3 pr-3 mr-3"
      color="primary"
      prepend-icon="mdi-plus"
      @click="() => onShowCombinationForm(null)"
    >
      Nova combinação
    </v-btn>

    <v-btn v-if="tab == 1" class="pl-3 pr-3 mr-3">
      Fatores de carga
    </v-btn>

    <v-btn class="pl-3 pr-3" @click="onClose">
      {{ t("buttons.close") }}
    </v-btn>
  </div>
</v-card>

<v-dialog v-model="dialogs.loadcases.form" persistent width="300">
    <v-card class="py-4 px-6">
      <v-card-title class="pb-7">
      </v-card-title>

      <v-card-text>
        <v-form ref="form" validate-on="submit">
          <v-row>
            <v-text-field
              v-model="formName"
              :rules="[validName]"
              :label="t('dialogs.materials.name')"
              prepend-inner-icon="mdi-tag"
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
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import useGlobalStore from "@/state/global";

const { t } = useI18n();
const store = useGlobalStore();

const tab = ref<number>(0);
const formName = ref<string>("");
const dialogs = ref({
  loadcases: {
    form: false,
    delete: false,
  },
  combinations: {
    form: false,
    delete: false,
  },
});

function onShowLoadcaseForm(value: object | null) {
  console.log(value);
}

function onShowCombinationForm(value: object | null) {
  console.log(value);
}

function onShowDelete(value: object) {
  console.log(value);
}

function onClose() {
  store.showDialog.loadcases = false;
}
</script>