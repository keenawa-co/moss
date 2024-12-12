import { LanguagePack } from "@/store/language";

import { invokeIpc, IpcResult } from "./tauri";

const getLocales = async (): Promise<IpcResult<LanguagePack[], string>> => {
  return await invokeIpc<LanguagePack[], string>("get_locales");
};

export default getLocales;
