import { create } from "zustand";

import i18n from "@/app/i18n";
import getLocales from "@/lib/backend/locales";

export interface LanguageCode {
  code: string;
  name: string;
  direction: string;
}

const LOCALSTORAGE_KEY = "language";

interface LanguageStore {
  currentLanguage: LanguageCode["code"];
  languages: LanguageCode[];
  setLanguage: (newLanguage: LanguageCode["code"]) => void;
  initializeLanguage: () => void;
  initializeLanguages: () => void;
}

export const useLanguageStore = create<LanguageStore>((set, get) => ({
  currentLanguage: (localStorage.getItem(LOCALSTORAGE_KEY) as LanguageCode["code"]) || "en",
  languages: [],
  setLanguage: (newLanguage) => {
    const { languages } = get();

    const isValidLanguage = languages.some(({ code }) => code === newLanguage);
    const validLanguage = isValidLanguage ? newLanguage : "en";

    localStorage.setItem(LOCALSTORAGE_KEY, validLanguage);
    i18n.changeLanguage(validLanguage);
    set({ currentLanguage: validLanguage });
  },
  initializeLanguage: async () => {
    const { languages } = get();

    const storedLanguage = localStorage.getItem(LOCALSTORAGE_KEY) as LanguageCode["code"];

    if (storedLanguage && languages.some(({ code }) => code === storedLanguage)) {
      i18n.changeLanguage(storedLanguage);
      set({ currentLanguage: storedLanguage });
    } else {
      localStorage.setItem(LOCALSTORAGE_KEY, "en");
      i18n.changeLanguage("en");
      set({ currentLanguage: "en" });
    }
  },
  initializeLanguages: async () => {
    try {
      const locales = await getLocales();

      if (locales.status === "ok") {
        set({ languages: locales.data });
      }
    } catch (error) {
      console.error("Error fetching locales:", error);
    }
  },
}));
