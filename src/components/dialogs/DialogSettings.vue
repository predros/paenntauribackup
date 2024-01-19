<template>
  <v-card class="py-4 px-6" style="min-width: 600px; min-height: 500px">
    <v-card-title class="pb-6">
      {{ t("dialogs.settings.title") }}
    </v-card-title>

    <v-card-text class="d-flex flex-row">
      <v-col class="mr-2" md="4">
        <v-row>
          <v-select
            v-model="currentSettings.locale"
            label="Locale"
            :items="[
              { title: t('dialogs.settings.locales.enUS'), value: 'en-US' },
              { title: t('dialogs.settings.locales.ptBR'), value: 'pt-BR' },
            ]"
          />
        </v-row>
        <v-row>
          <v-checkbox
            v-model="currentSettings.dark_theme"
            color="primary"
            disabled
            :label="t('dialogs.settings.darkTheme')"
          />
        </v-row>
        <v-row>
          <v-col>
            <v-row>
              <span class="text-h6">
                {{ t("dialogs.settings.gridSpacing") }}
              </span>
            </v-row>
            <v-row>
              <v-col>
                <v-text-field
                  v-model="currentSettings.grid_spacing.x"
                  :label="t('dialogs.settings.gridSpacings.x')"
                  :placeholder="`(${settings.getUnitName(UnitType.Length)})`"
                  :rules="[validNumber, isPositive]"
                />
              </v-col>
              <v-col>
                <v-text-field
                  v-model="currentSettings.grid_spacing.y"
                  :label="t('dialogs.settings.gridSpacings.y')"
                  :placeholder="`(${settings.getUnitName(UnitType.Length)})`"
                  :rules="[validNumber, isPositive]"
                />
              </v-col>
            </v-row>
          </v-col>
        </v-row>
      </v-col>

      <v-col class="ml-2">
        <v-row>
          <span class="text-h6 pb-6">
            {{ t("dialogs.settings.unitsTitle") }}
          </span>
        </v-row>
        <v-carousel
          :show-arrows="false"
          delimiter-icon="mdi-square"
          hide-delimiter-background
          color="primary"
        >
          <v-carousel-item>
            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.length") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.length"
                  :items="unitOptions.length"
                  :label="t('dialogs.settings.unit')"
                  density="comfortable"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.length[0]"
                  :items="unitPrecision"
                  :label="t('dialogs.settings.unitPrecision')"
                  density="comfortable"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.length[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.force") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.force"
                  :items="unitOptions.force"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.force[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.force[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.moment") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.moment"
                  :items="unitOptions.moment"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.moment[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.moment[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.load") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.load"
                  :items="unitOptions.load"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.load[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.load[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.angle") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.angle"
                  :items="unitOptions.angle"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.angle[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.angle[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>
          </v-carousel-item>

          <v-carousel-item>
            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.temperature") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.temperature"
                  :items="unitOptions.temperature"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.temperature[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.temperature[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.displacement") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.displacement"
                  :items="unitOptions.displacement"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.displacement[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.displacement[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.rotation") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.rotation"
                  :items="unitOptions.rotation"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.rotation[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.rotation[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.spring") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.spring"
                  :items="unitOptions.spring"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.spring[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.spring[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.torsionSpring") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.torsionSpring"
                  :items="unitOptions.torsionSpring"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.torsion_spring[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.torsion_spring[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>
          </v-carousel-item>

          <v-carousel-item>
            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.elasticity") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.elasticity"
                  :items="unitOptions.elasticity"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.elasticity[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.elasticity[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.thermal") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.thermal"
                  :items="unitOptions.thermal"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.thermal[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.thermal[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.inertia") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.inertia"
                  :items="unitOptions.inertia"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.inertia[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.inertia[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.area") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.area"
                  :items="unitOptions.area"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.area[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.area[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>

            <v-row>
              <v-col class="d-flex flex-column justify-center">
                <span class="text-body-2">
                  {{ t("dialogs.settings.units.dimension") }}
                </span>
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.units.dimension"
                  :items="unitOptions.dimension"
                  density="comfortable"
                  :label="t('dialogs.settings.unit')"
                />
              </v-col>
              <v-col>
                <v-select
                  v-model="currentSettings.unit_precision.dimension[0]"
                  :items="unitPrecision"
                  density="comfortable"
                  :label="t('dialogs.settings.unitPrecision')"
                />
              </v-col>
              <v-col>
                <v-checkbox
                  v-model="currentSettings.unit_precision.dimension[1]"
                  color="primary"
                  label="0e+0"
                  density="comfortable"
                />
              </v-col>
            </v-row>
          </v-carousel-item>
        </v-carousel>
      </v-col>
    </v-card-text>

    <div class="d-flex justify-end">
      <v-btn class="pl-3 pr-3 mr-3" color="primary" @click="onSubmitAndClose">
        {{ t("buttons.saveAndClose") }}
      </v-btn>

      <v-btn class="pl-3 pr-3 mr-3" @click="onSubmit">
        {{ t("buttons.apply") }}
      </v-btn>

      <v-btn class="pl-3 pr-3" @click="onClose">
        {{ t("buttons.cancel") }}
      </v-btn>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { reactive, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import useSettings from "@/state/settings";
import {
  ISettings,
  UnitType,
  UnitAngle,
  UnitLength,
  UnitForce,
  UnitStress,
  UnitTemperature,
  IUnitPrecision,
} from "@/types/units";
import { isValidNumber } from "@/helper/misc";

const { t } = useI18n();
const settings = useSettings();

const currentSettings = reactive({
  dark_theme: false,
  locale: "pt-BR",
  grid_spacing: {
    x: 100,
    y: 100,
  },
  units: {
    length: "",
    force: "",
    angle: "",
    temperature: "",
    moment: "",
    load: "",
    displacement: "",
    rotation: "",
    spring: "",
    torsionSpring: "",
    elasticity: "",
    thermal: "",
    inertia: "",
    area: "",
    dimension: "",
  },
  unit_precision: {
    length: [1, false],
    force: [1, false],
    angle: [1, false],
    temperature: [1, false],
    moment: [1, false],
    load: [1, false],
    displacement: [1, false],
    rotation: [1, false],
    spring: [1, false],
    torsion_spring: [1, false],
    elasticity: [1, false],
    thermal: [1, false],
    inertia: [1, false],
    area: [1, false],
    dimension: [1, false],
  } as IUnitPrecision,
});

const unitOptions = computed(() => {
  const length = ["cm", "mm", "m", "in", "ft"];
  const force = ["kN", "N", "kgf", "tf", "lbf"];

  const result = {
    length: [] as string[],
    force: [] as string[],
    angle: ["°", "rad"],
    temperature: ["°C", "°F", "K"],
    moment: [] as string[],
    load: [] as string[],
    displacement: [] as string[],
    rotation: ["°", "rad"],
    spring: [] as string[],
    torsionSpring: [] as string[],
    elasticity: ["MPa", "kPa", "Pa", "kN/cm²", "psi", "atm"],
    thermal: ["°C⁻¹", "°F⁻¹", "K⁻¹"],
    inertia: [] as string[],
    area: [] as string[],
    dimension: [] as string[],
  };

  length.forEach((lengthUnit) => {
    result.length.push(lengthUnit);
    result.displacement.push(lengthUnit);
    result.inertia.push(lengthUnit + "⁴");
    result.area.push(lengthUnit + "²");
    result.dimension.push(lengthUnit);
  });

  force.forEach((forceUnit) => {
    result.force.push(forceUnit);

    result.length.forEach((lengthUnit) => {
      result.moment.push(forceUnit + "." + lengthUnit);
      result.load.push(forceUnit + "/" + lengthUnit);
      result.spring.push(forceUnit + "/" + lengthUnit);
      result.torsionSpring.push(forceUnit + "." + lengthUnit + "/°");
      result.torsionSpring.push(forceUnit + "." + lengthUnit + "/rad");
    });
  });

  return result;
});

const unitPrecision = reactive([
  { title: "0", value: 0 },
  { title: "0.0", value: 1 },
  { title: "0.00", value: 2 },
  { title: "0.000", value: 3 },
  { title: "0.0000", value: 4 },
]);

onMounted(() => {
  const getSettings = settings.getSettings();

  currentSettings.dark_theme = getSettings.dark_theme;
  currentSettings.locale = getSettings.locale;
  currentSettings.grid_spacing.x = getSettings.grid_spacing[0];
  currentSettings.grid_spacing.y = getSettings.grid_spacing[1];

  currentSettings.unit_precision = getSettings.unit_precision;

  const units = getSettings.units;
  const options = unitOptions.value;

  currentSettings.units.angle = options.angle[units.angle];
  currentSettings.units.area = options.area[units.area];
  currentSettings.units.dimension = options.dimension[units.dimension];
  currentSettings.units.displacement = options.displacement[units.displacement];
  currentSettings.units.elasticity = options.elasticity[units.elasticity];
  currentSettings.units.force = options.force[units.force];
  currentSettings.units.inertia = options.inertia[units.inertia];
  currentSettings.units.length = options.length[units.length];
  currentSettings.units.rotation = options.rotation[units.rotation];
  currentSettings.units.temperature = options.temperature[units.temperature];
  currentSettings.units.thermal = options.thermal[units.thermal];

  currentSettings.units.load = options.load[5 * units.load[0] + units.load[1]];
  currentSettings.units.moment =
    options.moment[5 * units.moment[0] + units.moment[1]];
  currentSettings.units.spring =
    options.spring[5 * units.spring[0] + units.spring[1]];
  currentSettings.units.torsionSpring =
    options.torsionSpring[
      2 * (5 * units.torsion_spring[0] + units.torsion_spring[1]) +
        units.torsion_spring[2]
    ];
});

async function onSubmitAndClose(): Promise<void> {
  await onSubmit();
  onClose();
}

async function onSubmit(): Promise<void> {
  const gridTempX = settings.lengthToCm(currentSettings.grid_spacing.x);
  const gridTempY = settings.lengthToCm(currentSettings.grid_spacing.y);

  const units = unitOptions.value;
  const current = currentSettings.units;

  const angle = units.angle.indexOf(current.angle);
  const area = units.area.indexOf(current.area);
  const dimension = units.dimension.indexOf(current.dimension);
  const displacement = units.displacement.indexOf(current.displacement);
  const elasticity = units.elasticity.indexOf(current.elasticity);
  const force = units.force.indexOf(current.force);
  const inertia = units.inertia.indexOf(current.inertia);
  const length = units.length.indexOf(current.length);
  const rotation = units.rotation.indexOf(current.rotation);
  const temperature = units.temperature.indexOf(current.temperature);
  const thermal = units.thermal.indexOf(current.thermal);

  if (
    angle < 0 ||
    area < 0 ||
    dimension < 0 ||
    displacement < 0 ||
    elasticity < 0 ||
    force < 0 ||
    inertia < 0 ||
    length < 0 ||
    rotation < 0 ||
    temperature < 0 ||
    thermal < 0
  )
    return;

  const load = units.load.indexOf(current.load);
  if (load < 0) return;
  const load1 = load % 5;
  const load0 = load - load1;

  const moment = units.moment.indexOf(current.moment);
  if (moment < 0) return;
  const moment1 = moment % 5;
  const moment0 = (moment - moment1) / 5;

  const spring = units.spring.indexOf(current.spring);
  if (spring < 0) return;
  const spring1 = spring % 5;
  const spring0 = (spring - spring1) / 5;

  const torsionSpring = units.torsionSpring.indexOf(current.torsionSpring);
  if (torsionSpring < 0) return;
  const torsionSpring2 = torsionSpring % 2;
  const aux = (torsionSpring - torsionSpring2) / 2;
  const torsionSpring1 = aux % 5;
  const torsionSpring0 = (aux - torsionSpring1) / 5;

  const result: ISettings = {
    dark_theme: currentSettings.dark_theme,
    locale: currentSettings.locale,
    grid_spacing: [
      currentSettings.grid_spacing.x,
      currentSettings.grid_spacing.y,
    ],
    unit_precision: currentSettings.unit_precision as IUnitPrecision,
    units: {
      angle: angle as UnitAngle,
      area: area as UnitLength,
      dimension: dimension as UnitLength,
      displacement: displacement as UnitLength,
      elasticity: elasticity as UnitStress,
      force: force as UnitForce,
      inertia: inertia as UnitLength,
      load: [load0 as UnitForce, load1 as UnitLength],
      length: length as UnitLength,
      moment: [moment0 as UnitForce, moment1 as UnitLength],
      spring: [spring0 as UnitForce, spring1 as UnitLength],
      rotation: rotation as UnitAngle,
      temperature: temperature as UnitTemperature,
      thermal: thermal as UnitTemperature,
      torsion_spring: [
        torsionSpring0 as UnitForce,
        torsionSpring1 as UnitLength,
        torsionSpring2 as UnitAngle,
      ],
    },
  };

  await settings.saveSettings(result);

  currentSettings.grid_spacing.x = settings.lengthFromCm(gridTempX);
  currentSettings.grid_spacing.y = settings.lengthFromCm(gridTempY);
}

function onClose(): void {
  settings.showSettingsDialog = false;
}

function validNumber(value: string): boolean | string {
  return isValidNumber(value, true, true) || t("errors.validNumber");
}

function isPositive(value: string): boolean | string {
  return isValidNumber(value, false, false) || t("errors.isPositive");
}
</script>
