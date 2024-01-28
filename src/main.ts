import { createApp } from "vue";
import { createPinia } from "pinia";

import { useI18n } from "vue-i18n";
import i18n from "./i18n/i18n";

import VueKonva from "vue-konva";

import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";
import { createVuetify } from "vuetify";
import { createVueI18nAdapter } from "vuetify/locale/adapters/vue-i18n";

import App from "./App.vue";
import { type ThemeDefinition } from "vuetify/dist/vuetify.js";

const pinia = createPinia();

const lightTheme: ThemeDefinition = {
  dark: false,
  colors: {
    primary: "#00796B",
    secondary: "#F9AA33",
    background: "#F7F7F7",
  },
};

const vuetify = createVuetify({
  icons: {
    defaultSet: "mdi", // This is already the default value - only for display purposes
  },
  theme: {
    defaultTheme: "lightTheme",
    themes: {
      lightTheme,
    },
  },
  locale: {
    adapter: createVueI18nAdapter({ i18n, useI18n }),
  },
});

const app = createApp(App);

app.use(vuetify);
app.use(pinia);
app.use(VueKonva);
app.use(i18n);

app.mount("#app");
