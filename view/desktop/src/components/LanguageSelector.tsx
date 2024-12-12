import React from "react";

import { LanguageCode, useLanguageStore } from "@/store/language";

export const LanguageSelector = () => {
  const { currentLanguage, setLanguage, languages } = useLanguageStore();

  const onChangeLanguage = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setLanguage(e.target.value as LanguageCode["code"]);
  };

  return (
    <select
      className="bg-purple-300 text-[rgba(var(--color-primary))]"
      value={currentLanguage}
      onChange={onChangeLanguage}
    >
      {languages.map(({ code, name }) => (
        <option key={code} value={code}>
          {name}
        </option>
      ))}
    </select>
  );
};

export default LanguageSelector;
