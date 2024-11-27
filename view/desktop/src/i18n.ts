import i18next from "i18next";
import { initReactI18next } from "react-i18next";
import resourcesToBackend from "i18next-resources-to-backend";

export const defaultNS = "ns1";

i18next
  .use(
    resourcesToBackend((language: string, namespace: string) => {
      return import(`../../../packages/moss_lang/locales/${language}/${namespace}.json`);
    })
  )
  .use(initReactI18next)
  .init({
    debug: false,
    fallbackLng: "en",
    defaultNS,
    ns: "ns1",
  });

export default i18next;
