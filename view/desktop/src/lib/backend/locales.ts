import { LanguageCode } from "@/store/language";

import { invokeIpc, IpcResult } from "./tauri";

const getLocales = async (): Promise<IpcResult<LanguageCode[], string>> => {
  return await invokeIpc<LanguageCode[], string>("get_locales");
};

export default getLocales;
