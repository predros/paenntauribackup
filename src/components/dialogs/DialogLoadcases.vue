<template>
  <v-card
    class="py-4 px-6"
    style="min-width: 600px; min-height: 500px"
  >
    <v-card-title class="pb-6">
      {{ t("dialogs.loadcases.title") }}
    </v-card-title>

    <v-card-text>
      <v-tabs
        v-model="tab"
        background-color="primary"
      >
        <v-tab :value="0">{{ t("dialogs.loadcases.loadcases.title") }}</v-tab>
        <v-tab :value="1">{{
          t("dialogs.loadcases.combinations.title")
        }}</v-tab>
      </v-tabs>

      <v-window
        v-model="tab"
        class="pt-3"
      >
        <v-window-item :value="0">
          <v-data-table
            :headers="[
              {
                title: t('dialogs.loadcases.loadcases.name'),
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
            hide-no-data
          >
            <template #item="{ item }">
              <tr>
                <td>{{ item.name }}</td>
                <td>
                  <v-btn
                    icon
                    elevation="0"
                    @click="onShowFormLoadcase(item)"
                  >
                    <v-icon>mdi-pencil</v-icon>
                    <v-tooltip
                      location="bottom"
                      activator="parent"
                    >
                      {{ t("buttons.edit") }}
                    </v-tooltip>
                  </v-btn>
                  <v-btn
                    icon
                    elevation="0"
                    @click="onShowDeleteLoadcase(item)"
                  >
                    <v-icon>mdi-trash-can</v-icon>
                    <v-tooltip
                      location="bottom"
                      activator="parent"
                    >
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
                title: t('dialogs.loadcases.combinations.name'),
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
            hide-no-data
          >
            <template #item="{ item }">
              <tr>
                <td>{{ item.name }}</td>
                <td>
                  <v-btn
                    icon
                    elevation="0"
                    @click="onShowFormCombination(item)"
                  >
                    <v-icon>mdi-pencil</v-icon>
                    <v-tooltip
                      location="bottom"
                      activator="parent"
                    >
                      {{ t("buttons.edit") }}
                    </v-tooltip>
                  </v-btn>
                  <v-btn
                    icon
                    elevation="0"
                    @click="onShowDeleteCombination(item)"
                  >
                    <v-icon>mdi-trash-can</v-icon>
                    <v-tooltip
                      location="bottom"
                      activator="parent"
                    >
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
        @click="() => onShowFormLoadcase(null)"
      >
        {{ t("buttons.loadcaseNew") }}
      </v-btn>
      <v-btn
        v-else
        class="pl-3 pr-3 mr-3"
        color="primary"
        prepend-icon="mdi-plus"
        @click="() => onShowFormCombination(null)"
      >
        {{ t("buttons.combinationNew") }}
      </v-btn>

      <v-btn
        v-if="tab == 1"
        :disabled="store.combinationsList.length < 1"
        class="pl-3 pr-3 mr-3"
        @click="onShowFactors"
      >
        {{ t("buttons.combinationFactors") }}
      </v-btn>

      <v-btn
        class="pl-3 pr-3"
        @click="onClose"
      >
        {{ t("buttons.close") }}
      </v-btn>
    </div>
  </v-card>

  <v-dialog
    v-model="dialogs.loadcases.form"
    persistent
    width="300"
  >
    <v-card class="py-4 px-6">
      <v-card-title class="pb-7">
        {{
          current.loadcase.id === null
            ? t("dialogs.loadcases.loadcases.loadcaseNew")
            : t("dialogs.loadcases.loadcases.editing", [current.loadcase.name])
        }}
      </v-card-title>

      <v-card-text>
        <v-form
          ref="loadcaseForm"
          validate-on="submit"
        >
          <v-row>
            <v-text-field
              v-model="form.loadcase"
              :rules="[validName, uniqueLoadcaseName]"
              :label="t('dialogs.loadcases.loadcases.name')"
              prepend-inner-icon="mdi-tag"
            />
          </v-row>
        </v-form>
      </v-card-text>

      <div class="d-flex justify-end">
        <v-btn
          class="pl-3 pr-3 mr-3"
          color="primary"
          @click="onSubmitLoadcase"
        >
          {{ t("buttons.save") }}
        </v-btn>

        <v-btn
          class="pl-3 pr-3"
          @click="onDialogClose"
        >
          {{ t("buttons.cancel") }}
        </v-btn>
      </div>
    </v-card>
  </v-dialog>

  <v-dialog
    v-model="dialogs.combinations.form"
    persistent
    width="300"
  >
    <v-card class="py-4 px-6">
      <v-card-title class="pb-7">
        {{
          current.combination.id === null
            ? t("dialogs.loadcases.combinations.new")
            : t("dialogs.loadcases.combinations.editing", [
                current.combination.name,
              ])
        }}
      </v-card-title>

      <v-card-text>
        <v-form
          ref="combinationForm"
          validate-on="submit"
        >
          <v-row>
            <v-text-field
              v-model="form.combination"
              :rules="[validName, uniqueCombinationName]"
              :label="t('dialogs.loadcases.combinations.name')"
              prepend-inner-icon="mdi-tag"
            />
          </v-row>
        </v-form>
      </v-card-text>

      <div class="d-flex justify-end">
        <v-btn
          class="pl-3 pr-3 mr-3"
          color="primary"
          @click="onSubmitCombination"
        >
          {{ t("buttons.save") }}
        </v-btn>

        <v-btn
          class="pl-3 pr-3"
          @click="onDialogClose"
        >
          {{ t("buttons.cancel") }}
        </v-btn>
      </div>
    </v-card>
  </v-dialog>

  <v-dialog
    v-model="dialogs.loadcases.delete"
    persistent
  >
    <v-card class="py-4 px-6">
      <v-card-title>
        {{
          current.loadcase.id === null
            ? t("errors.ERROR")
            : t("dialogs.loadcases.loadcases.deleting", [current.loadcase.name])
        }}
      </v-card-title>

      <v-card-text>
        {{
          t("dialogs.loadcases.loadcases.areYouSure", [current.loadcase.name])
        }}
      </v-card-text>

      <div class="d-flex justify-end">
        <v-btn
          v-if="current.loadcase.id !== null"
          class="pl-3 pr-3 mr-3"
          color="primary"
          @click="onDeleteLoadcase"
        >
          {{ t("buttons.delete") }}
        </v-btn>

        <v-btn
          class="pl-3 pr-3"
          @click="onDialogClose"
        >
          {{ t("buttons.cancel") }}
        </v-btn>
      </div>
    </v-card>
  </v-dialog>

  <v-dialog
    v-model="dialogs.combinations.delete"
    persistent
  >
    <v-card class="py-4 px-6">
      <v-card-title>
        {{
          current.combination.id === null
            ? t("errors.ERROR")
            : t("dialogs.loadcases.combinations.deleting", [
                current.combination.name,
              ])
        }}
      </v-card-title>

      <v-card-text>
        {{
          t("dialogs.loadcases.combinations.areYouSure", [
            current.combination.name,
          ])
        }}
      </v-card-text>

      <div class="d-flex justify-end">
        <v-btn
          v-if="current.loadcase.id !== null"
          class="pl-3 pr-3 mr-3"
          color="primary"
          @click="onDeleteCombination"
        >
          {{ t("buttons.delete") }}
        </v-btn>

        <v-btn
          class="pl-3 pr-3"
          @click="onDialogClose"
        >
          {{ t("buttons.cancel") }}
        </v-btn>
      </div>
    </v-card>
  </v-dialog>

  <v-dialog
    v-model="dialogs.combinations.factors"
    persistent
    :max-width="700"
  >
    <DialogCombinationFactors @close="onDialogClose" />
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { useI18n } from "vue-i18n";
import useGlobalStore from "@/state/global";
import { ICombination, ILoadcase } from "@/types/types";
import { VForm } from "vuetify/components";
import DialogCombinationFactors from "./DialogCombinationFactors.vue";

const { t } = useI18n();
const store = useGlobalStore();

const tab = ref<number>(0);

const loadcaseForm = ref<VForm>();
const combinationForm = ref<VForm>();

const form = reactive({
  loadcase: "",
  combination: "",
});

const dialogs = reactive({
  loadcases: {
    form: false,
    delete: false,
  },
  combinations: {
    factors: false,
    form: false,
    delete: false,
  },
});

const current = reactive({
  loadcase: {
    id: null as number | null,
    name: "",
  },
  combination: {
    id: null as number | null,
    name: "",
  },
});

function onShowFactors() {
  dialogs.combinations.factors = true;
}

function onShowFormLoadcase(value: ILoadcase | null) {
  if (value !== null) {
    current.loadcase.id = value.id;
    current.loadcase.name = value.name;
    form.loadcase = value.name;
  } else {
    current.loadcase.id = null;
    current.loadcase.name = "";
    form.loadcase = "";
  }

  dialogs.loadcases.form = true;
}

function onShowFormCombination(value: ICombination | null) {
  if (value !== null) {
    current.combination.id = value.id;
    current.combination.name = value.name;
    form.combination = value.name;
  } else {
    current.combination.id = null;
    current.combination.name = "";
    form.combination = "";
  }

  dialogs.combinations.form = true;
}

function onShowDeleteLoadcase(value: ILoadcase) {
  if (store.loadcasesList.length < 2) {
    store.appAlert(t("alerts.singleLoadcase"));
  } else {
    current.loadcase.id = value.id;
    current.loadcase.name = value.name;
    dialogs.loadcases.delete = true;
  }
}

function onShowDeleteCombination(value: ILoadcase) {
  current.combination.id = value.id;
  current.combination.name = value.name;
  dialogs.combinations.delete = true;
}

function onDialogClose() {
  form.combination = "";
  form.loadcase = "";

  dialogs.loadcases.delete = false;
  dialogs.loadcases.form = false;
  dialogs.combinations.delete = false;
  dialogs.combinations.form = false;
  dialogs.combinations.factors = false;
}

async function onSubmitLoadcase() {
  if (!loadcaseForm.value) {
    return;
  }

  const { valid } = await loadcaseForm.value.validate();

  if (valid) {
    if (current.loadcase.id === null) {
      await store.loadcaseNew(form.loadcase);
    } else {
      await store.loadcaseUpdate(current.loadcase.id, form.loadcase);
    }
    onDialogClose();
  }
}

async function onSubmitCombination() {
  if (!combinationForm.value) {
    return;
  }

  const { valid } = await combinationForm.value.validate();

  if (valid) {
    if (current.combination.id === null) {
      await store.combinationNew(form.combination);
    } else {
      await store.combinationUpdate(current.combination.id, form.combination);
    }
    onDialogClose();
  }
}

async function onDeleteLoadcase() {
  if (current.loadcase.id !== null) {
    await store.loadcaseDelete(current.loadcase.id);
  }
  dialogs.loadcases.delete = false;
}

async function onDeleteCombination() {
  if (current.combination.id !== null) {
    await store.combinationDelete(current.combination.id);
  }
  dialogs.combinations.delete = false;
}

function onClose() {
  store.dialogs.loadcases = false;
}

function validName(value: string): boolean | string {
  return value.trim() != "" || t("errors.validName");
}

function uniqueLoadcaseName(value: string): boolean | string {
  const exists = store.loadcasesList.some(
    (x) => x.name.toLowerCase() == value.toLowerCase(),
  );

  return !exists || t("errors.uniqueLoadcaseName");
}

function uniqueCombinationName(value: string): boolean | string {
  const exists = store.combinationsList.some(
    (x) => x.name.toLowerCase() == value.toLowerCase(),
  );

  return !exists || t("errors.uniqueCombinationName");
}
</script>
