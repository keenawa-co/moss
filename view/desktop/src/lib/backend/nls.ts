import { BackendModule, ReadCallback } from "i18next";
import { getTranslations } from "@/api/locale";

const I18nTauriBackend: BackendModule = {
  type: "backend",
  init: () => {},
  read: async (language: string, namespace: string, callback: ReadCallback) => {
    console.log(`getTranslations ${language} ${namespace}`);
    const result = await getTranslations(language, namespace);
    if (result.status === "ok") {
      callback(null, result.data);
    } else {
      callback(result.error, false);
    }
  },
};

export default I18nTauriBackend;
