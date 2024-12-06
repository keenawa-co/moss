import { BackendModule, ReadCallback } from "i18next";

import { invokeIpc, IpcResult } from "./tauri";

interface I18nDictionary {
  [key: string]: string;
}

const getTranslations = (language: string, namespace: string): Promise<IpcResult<Record<string, string>, string>> => {
  return invokeIpc<I18nDictionary, string>("get_translations", { language, namespace });
};

const I18nTauriBackend: BackendModule = {
  type: "backend",
  init: () => {},
  read: async (language: string, namespace: string, callback: ReadCallback) => {
    const result = await getTranslations(language, namespace);
    if (result.status === "ok") {
      callback(null, result.data);
    } else {
      callback(result.error, false);
    }
  },
};

export default I18nTauriBackend;
