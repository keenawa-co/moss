import React from "react";
import { useTranslation } from "react-i18next";
import { LANGUAGES } from "@/constants/index";

export const LanguageSelector: React.FC = () => {
  const { i18n } = useTranslation();

  const onChangeLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const lang_code = e.target.value;
    i18n.changeLanguage(lang_code);
  };

  return (
    <select className="bg-purple-300 text-primary" defaultValue={i18n.language} onChange={onChangeLang}>
      {LANGUAGES.map(({ code, label }) => (
        <option key={code} value={code}>
          {label}
        </option>
      ))}
    </select>
  );
};

export default LanguageSelector;
