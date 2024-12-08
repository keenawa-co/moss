import { atomWithStorage } from "jotai/utils";

import i18n from "@/app/i18n";

// FIXME remove all the hardcoded values
export const LANGUAGES = [
  { label: "English", code: "en" },
  { label: "Deutsche", code: "de" },
  { label: "Русский", code: "ru" },
] as const;

export type LanguageCodes = (typeof LANGUAGES)[number]["code"];

const LOCALSTORAGE_KEY = "language";

export const languageAtom = atomWithStorage<LanguageCodes>(LOCALSTORAGE_KEY, "en", {
  getItem() {
    return localStorage.getItem(LOCALSTORAGE_KEY) as LanguageCodes;
  },
  setItem(_, newLanguage) {
    const isValidLanguage = LANGUAGES.some(({ code }) => code === newLanguage);

    localStorage.setItem(LOCALSTORAGE_KEY, isValidLanguage ? newLanguage : "en");
    i18n.changeLanguage(isValidLanguage ? newLanguage : "en");
  },
  removeItem(key) {
    localStorage.removeItem(key);
  },
  subscribe(_, callback) {
    if (typeof window === "undefined" || typeof window.addEventListener === "undefined") {
      return () => {};
    }

    const storageHandler = (e: StorageEvent) => {
      if (e.storageArea === localStorage && e.key === LOCALSTORAGE_KEY) {
        callback(localStorage.getItem(LOCALSTORAGE_KEY) as LanguageCodes);
      }
    };

    window.addEventListener("storage", storageHandler);

    return () => {
      window.removeEventListener("storage", storageHandler);
    };
  },
});

export const initializeLanguage = () => {
  const storedLanguage = localStorage.getItem(LOCALSTORAGE_KEY) as LanguageCodes;

  if (storedLanguage && LANGUAGES.some(({ code }) => code === storedLanguage)) {
    i18n.changeLanguage(storedLanguage);
  } else {
    localStorage.setItem(LOCALSTORAGE_KEY, "en");
    i18n.changeLanguage("en");
  }
};
