import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import I18nTauriBackend from "../lib/backend/nls";

i18next
  .use(I18nTauriBackend)
  .use(initReactI18next)
  .init({
    lng: "en",
    fallbackLng: "en",
    ns: ["ns1"],
    defaultNS: "ns1",
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: true,
    },
  });

export default i18next;
