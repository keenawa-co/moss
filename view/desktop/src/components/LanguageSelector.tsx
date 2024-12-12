import React from "react";

import { useLanguageStore } from "@/store/language";
import { useChangeLanguagePack } from "@/hooks/useChangeLanguagePack.ts";

export const LanguageSelector = () => {
  const { currentLanguageCode, setLanguageCode, languagePacks } = useLanguageStore();
  const { mutate: mutateChangeLanguagePack } = useChangeLanguagePack();

  const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedLanguageCode = e.target.value;
    const selectedLanguagePack = languagePacks?.find((languagePack) => languagePack.code === selectedLanguageCode);

    if (selectedLanguagePack) {
      mutateChangeLanguagePack(selectedLanguagePack, {
        onSuccess: () => {
          setLanguageCode(selectedLanguageCode);
        },
        onError: (error: Error) => {
          console.error("Error changing locale:", error);
        },
      });
    }
  };

  return (
    <select
      className="bg-purple-300 text-[rgba(var(--color-primary))]"
      value={currentLanguageCode || "en"}
      onChange={handleChange}
    >
      {languagePacks?.map((languagePack) => (
        <option key={languagePack.code} value={languagePack.code}>
          {languagePack.name}
        </option>
      ))}
    </select>
  );
};

export default LanguageSelector;
