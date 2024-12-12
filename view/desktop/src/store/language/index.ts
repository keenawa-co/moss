import { create } from "zustand";

import i18n from "@/app/i18n";
import getLocales from "@/lib/backend/locales";

export interface LanguagePack {
  code: string;
  name: string;
  direction: string;
}

const LOCALSTORAGE_KEY = "language";

interface LanguageStore {
  currentLanguageCode: LanguagePack["code"];
  languagePacks: LanguagePack[];
  setLanguageCode: (newLanguage: LanguagePack["code"]) => void;
  initializeLanguage: () => void;
  initializeLanguages: () => void;
}

export const useLanguageStore = create<LanguageStore>((set, get) => ({
  currentLanguageCode: (localStorage.getItem(LOCALSTORAGE_KEY) as LanguagePack["code"]) || "en",
  languagePacks: [],
  setLanguageCode: (newLanguage) => {
    const { languagePacks } = get();

    const isValidLanguage = languagePacks.some(({ code }) => code === newLanguage);
    const validLanguageCode = isValidLanguage ? newLanguage : "en";

    localStorage.setItem(LOCALSTORAGE_KEY, validLanguageCode);
    i18n.changeLanguage(validLanguageCode);
    set({ currentLanguageCode: validLanguageCode });
  },
  initializeLanguage: async () => {
    const { languagePacks } = get();

    const storedLanguageCode = localStorage.getItem(LOCALSTORAGE_KEY) as LanguagePack["code"];

    if (storedLanguageCode && languagePacks.some(({ code }) => code === storedLanguageCode)) {
      i18n.changeLanguage(storedLanguageCode);
      set({ currentLanguageCode: storedLanguageCode });
    } else {
      localStorage.setItem(LOCALSTORAGE_KEY, "en");
      i18n.changeLanguage("en");
      set({ currentLanguageCode: "en" });
    }
  },
  initializeLanguages: async () => {
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
