import { useAtom } from "jotai";
import React from "react";
import { languageAtom, LanguageCodes, LANGUAGES } from "@/atoms/langAtom";

export const LanguageSelector = () => {
  const [language, setLanguage] = useAtom(languageAtom);

  const onChangeLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setLanguage(e.target.value as LanguageCodes);
  };

  return (
    <select className="bg-purple-300 text-[rgba(var(--color-primary))]" value={language} onChange={onChangeLang}>
      {LANGUAGES.map(({ code, label }) => (
        <option key={code} value={code}>
          {label}
        </option>
      ))}
    </select>
  );
};

export default LanguageSelector;
