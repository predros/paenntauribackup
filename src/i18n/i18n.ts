import { createI18n } from "vue-i18n";
import ptBR from "./ptBR";
import enUS from "./enUS";

export default createI18n({
  legacy: false,
  allowComposition: true,
  locale: "pt-BR",
  messages: {
    "en-US": enUS,
    "pt-BR": ptBR,
  },
});
