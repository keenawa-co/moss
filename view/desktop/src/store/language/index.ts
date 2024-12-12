import { create } from "zustand";

import i18n from "@/app/i18n";

// FIXME remove all the hardcoded values
export const LANGUAGES = [
  { label: "English", code: "en" },
  { label: "Deutsche", code: "de" },
  { label: "Русский", code: "ru" },
] as const;

export type LanguageCodes = (typeof LANGUAGES)[number]["code"];

const LOCALSTORAGE_KEY = "language";

interface LanguageStore {
  language: LanguageCodes;
  setLanguage: (newLanguage: LanguageCodes) => void;
  initializeLanguage: () => void;
}

export const useLanguageStore = create<LanguageStore>((set) => ({
  language: (localStorage.getItem(LOCALSTORAGE_KEY) as LanguageCodes) || "en",
  setLanguage: (newLanguage) => {
    const isValidLanguage = LANGUAGES.some(({ code }) => code === newLanguage);

    const validLanguage = isValidLanguage ? newLanguage : "en";
    localStorage.setItem(LOCALSTORAGE_KEY, validLanguage);
    i18n.changeLanguage(validLanguage);

    set({ language: validLanguage });
  },
  initializeLanguage: () => {
    const storedLanguage = localStorage.getItem(LOCALSTORAGE_KEY) as LanguageCodes;

    if (storedLanguage && LANGUAGES.some(({ code }) => code === storedLanguage)) {
      i18n.changeLanguage(storedLanguage);
      set({ language: storedLanguage });
    } else {
      localStorage.setItem(LOCALSTORAGE_KEY, "en");
      i18n.changeLanguage("en");
      set({ language: "en" });
    }
  },
}));
