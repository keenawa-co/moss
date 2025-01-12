import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";

export interface GetTranslationsOptions {
  enableCache: boolean;
}

interface I18nDictionary {
  [key: string]: string;
}

export const getTranslations = async (
  language: string,
  namespace: string,
  opts?: GetTranslationsOptions
): Promise<IpcResult<I18nDictionary, string>> => {
  return await invokeTauriIpc<I18nDictionary, string>("get_translations", {
    language,
    namespace,
    opts,
  });
};
