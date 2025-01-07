import i18n from "@/app/i18n";
import { LocaleDescriptor } from "@repo/moss-desktop";

export const applyLanguagePack = (languagePack: LocaleDescriptor) => {
  i18n.changeLanguage(languagePack.code);
};
