import { create } from "zustand";

import i18n from "@/app/i18n";
import getLocales from "@/lib/backend/locales";
import { LocaleDescriptor } from "@repo/moss-desktop";

interface LanguageStore {
  currentLanguageCode: LocaleDescriptor["code"] | null;
  languagePacks: LocaleDescriptor[];
  setLanguageCode: (newLanguage: LocaleDescriptor["code"]) => void;
  initializeLanguages: () => void;
}

export const useLanguageStore = create<LanguageStore>((set, get) => ({
  currentLanguageCode: "en",
  languagePacks: [],
  setLanguageCode: (newLanguageCode) => {
    const { languagePacks, currentLanguageCode } = get();

    if (newLanguageCode === currentLanguageCode) return;

    const isValidLanguage = languagePacks.some(({ code }) => code === newLanguageCode);
    const validLanguageCode = isValidLanguage ? newLanguageCode : "en";

    i18n.changeLanguage(validLanguageCode);
    set({ currentLanguageCode: validLanguageCode });
  },
  initializeLanguages: async () => {
    console.log("initializeLanguages");
    try {
      const locales = await getLocales();

      if (locales.status === "ok") {
        set({ languagePacks: locales.data });
      }
    } catch (error) {
      console.error("Error fetching locales:", error);
    }
  },
}));
