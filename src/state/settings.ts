import { defineStore } from "pinia";
import { computed, reactive, ref } from "vue";
import {
  type ISettings,
  type IUnitPrecision,
  type IUnitSettings,
  UnitAngle,
  UnitTemperature,
  UnitType,
} from "@/types/units";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api";
import useGlobalStore from "@/state/global";

export default defineStore("settings", () => {
  const { t, locale } = useI18n();

  const store = useGlobalStore();

  const showSettingsDialog = ref<boolean>(false);
  const darkTheme = ref<boolean>(false);
  const gridSpacing = reactive({ x: 100, y: 100 });
  const currentLocale = computed(() => locale.value);

  const unitStrings = [
    ["cm", "mm", "m", "in", "ft"], // Length
    ["kN", "N", "kgf", "tf", "lbf"], // Force
    ["°", "rad"], // Angle
    ["°C", "°F", "K"], // Temperature
    ["MPa", "kPa", "Pa", "kN/cm²", "psi", "atm"], // Stress/elasticity
  ];

  const units = ref<IUnitSettings | undefined>();

  const unitPrecision = ref<IUnitPrecision | undefined>();

  const unitLocales = computed(() => {
    type LocaleType = Record<string, Intl.NumberFormat>;
    const result: LocaleType = {};

    if (unitPrecision.value == undefined) {
      return result;
    }

    Object.entries(unitPrecision.value).forEach((pair) => {
      result[pair[0]] = new Intl.NumberFormat(currentLocale.value, {
        maximumFractionDigits: pair[1][0],
        useGrouping: false,
      });
    });

    return result;
  });

  function getUnitName(unitType: UnitType): string {
    if (units.value == undefined) {
      return "";
    }

    switch (unitType) {
      case UnitType.Angle:
        return unitStrings[2][units.value.angle];
      case UnitType.Area:
        return unitStrings[0][units.value.area] + "²";
      case UnitType.Dimension:
        return unitStrings[0][units.value.dimension];
      case UnitType.Displacement:
        return unitStrings[0][units.value.displacement];
      case UnitType.Elasticity:
        return unitStrings[4][units.value.elasticity];
      case UnitType.Force:
        return unitStrings[1][units.value.force];
      case UnitType.Inertia:
        return unitStrings[0][units.value.inertia] + "⁴";
      case UnitType.Length:
        return unitStrings[0][units.value.length];
      case UnitType.Load:
        return (
          unitStrings[1][units.value.load[0]] +
          "/" +
          unitStrings[0][units.value.load[1]]
        );
      case UnitType.Moment:
        return (
          unitStrings[1][units.value.moment[0]] +
          "." +
          unitStrings[0][units.value.moment[1]]
        );
      case UnitType.Rotation:
        return unitStrings[2][units.value.rotation];
      case UnitType.Spring:
        return (
          unitStrings[1][units.value.spring[0]] +
          "/" +
          unitStrings[0][units.value.spring[1]]
        );
      case UnitType.Temperature:
        return unitStrings[3][units.value.temperature];
      case UnitType.Thermal:
        return unitStrings[3][units.value.thermal] + "⁻¹";
      case UnitType.TorsionSpring:
        return (
          unitStrings[1][units.value.torsionSpring[0]] +
          "." +
          unitStrings[0][units.value.torsionSpring[1]] +
          "/" +
          unitStrings[2][units.value.torsionSpring[2]]
        );
    }
  }

  function formatUnit(
    value: number,
    unitType: UnitType,
    includeUnit: boolean = true,
  ): string {
    if (units.value == undefined || unitPrecision.value == undefined) {
      return "";
    }

    let converted = value;
    if (unitType == UnitType.Length) {
      converted = lengthFromCm(value);
    }

    let precision: [number, boolean];
    let unitLocale: Intl.NumberFormat;

    switch (unitType) {
      case UnitType.Angle:
        precision = unitPrecision.value.angle;
        unitLocale = unitLocales.value.angle;
        break;
      case UnitType.Area:
        precision = unitPrecision.value.area;
        unitLocale = unitLocales.value.area;
        break;
      case UnitType.Dimension:
        precision = unitPrecision.value.dimension;
        unitLocale = unitLocales.value.dimension;
        break;
      case UnitType.Displacement:
        precision = unitPrecision.value.displacement;
        unitLocale = unitLocales.value.displacement;
        break;
      case UnitType.Elasticity:
        precision = unitPrecision.value.elasticity;
        unitLocale = unitLocales.value.elasticity;
        break;
      case UnitType.Force:
        precision = unitPrecision.value.force;
        unitLocale = unitLocales.value.force;
        break;
      case UnitType.Inertia:
        precision = unitPrecision.value.inertia;
        unitLocale = unitLocales.value.inertia;
        break;
      case UnitType.Length:
        precision = unitPrecision.value.length;
        unitLocale = unitLocales.value.length;
        break;
      case UnitType.Load:
        precision = unitPrecision.value.load;
        unitLocale = unitLocales.value.load;
        break;
      case UnitType.Moment:
        precision = unitPrecision.value.moment;
        unitLocale = unitLocales.value.moment;
        break;
      case UnitType.Rotation:
        precision = unitPrecision.value.rotation;
        unitLocale = unitLocales.value.rotation;
        break;
      case UnitType.Spring:
        precision = unitPrecision.value.spring;
        unitLocale = unitLocales.value.spring;
        break;
      case UnitType.Temperature:
        precision = unitPrecision.value.temperature;
        unitLocale = unitLocales.value.temperature;
        break;
      case UnitType.Thermal:
        precision = unitPrecision.value.thermal;
        unitLocale = unitLocales.value.thermal;
        break;
      case UnitType.TorsionSpring:
        precision = unitPrecision.value.torsionSpring;
        unitLocale = unitLocales.value.torsion_spring;
        break;
    }

    let result: string;

    if (precision[1]) {
      result = converted.toExponential(precision[0]);
      if (currentLocale.value == "pt-BR") {
        const split = result.split(".");
        result = split[0] + "," + split[1];
      }
    } else {
      result = unitLocale.format(converted);
    }

    if (includeUnit) {
      let unit: string;
      if (
        (unitType == UnitType.Temperature &&
          units.value.temperature != UnitTemperature.Kelvin) ||
        (unitType == UnitType.Angle && units.value.angle == UnitAngle.Degree) ||
        (unitType == UnitType.Rotation &&
          units.value.rotation == UnitAngle.Degree)
      ) {
        unit = getUnitName(unitType);
      } else {
        unit = " " + getUnitName(unitType);
      }

      result += unit;
    }

    return result;
  }

  function lengthFromCm(value: number): number {
    const ratios = [1, 10, 0.01, 0.393701, 0.0328084];

    if (units.value == undefined) {
      return value;
    }

    return value * ratios[units.value.length];
  }

  function lengthToCm(value: number): number {
    const ratios = [1, 10, 0.01, 0.393701, 0.0328084];

    if (units.value == undefined) {
      return value;
    }

    return value / ratios[units.value.length];
  }

  async function fetchSettings(): Promise<void> {
    const response = await invoke("settings_get").catch((e: string[]) => {
      store.appAlert(t(e[0], [e[1]]));
    });

    const result = response as ISettings;
    darkTheme.value = result.darkTheme;
    gridSpacing.x = result.gridSpacing[0];
    gridSpacing.y = result.gridSpacing[1];
    locale.value = result.locale;
    units.value = result.units;
    unitPrecision.value = result.unitPrecision;
  }

  async function saveSettings(settings: ISettings): Promise<void> {
    settings.gridSpacing[0] = lengthToCm(settings.gridSpacing[0]);
    settings.gridSpacing[1] = lengthToCm(settings.gridSpacing[1]);

    await invoke("settings_save", { settings }).catch((e: string[]) => {
      store.appAlert(t(e[0], [e[1]]));
    });
    locale.value = settings.locale;

    await fetchSettings();
  }

  function getSettings(): ISettings {
    return {
      darkTheme: darkTheme.value,
      locale: currentLocale.value,
      gridSpacing: [lengthFromCm(gridSpacing.x), lengthFromCm(gridSpacing.y)],
      units: JSON.parse(JSON.stringify(units.value)),
      unitPrecision: JSON.parse(JSON.stringify(unitPrecision.value)),
    };
  }

  return {
    showSettingsDialog,
    currentLocale,
    darkTheme,
    gridSpacing,
    locale,

    lengthFromCm,
    lengthToCm,
    saveSettings,
    fetchSettings,
    getUnitName,
    formatUnit,
    getSettings,
  };
});
