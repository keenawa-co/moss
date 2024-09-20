import React from "react";
import { useTranslation } from "react-i18next";
import { LANGUAGES } from "@/constants/index";

export const LanguageSelector: React.FC = () => {
  const { i18n } = useTranslation();
  const [language, setLanguage] = React.useState(() => localStorage.getItem("language") || i18n.language);

  const onChangeLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const langCode = e.target.value;
    setLanguage(langCode);
    localStorage.setItem("language", langCode);
    i18n.changeLanguage(langCode);
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
