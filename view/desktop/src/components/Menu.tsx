// TODO: Info
// Trans component can also be used for translation
import { useState, useEffect } from "react";
import { Trans, useTranslation } from "react-i18next";
import { NavLink } from "react-router-dom";
import { LANGUAGES } from "@/constants/index";

const isActive = ({ isActive }: any) => `link ${isActive ? "active" : ""}`;

export const Menu = () => {
  const { i18n, t } = useTranslation();
  const themes = ["blue", "black", "orange", "purple", "green"];

  const [theme, setTheme] = useState<string | null>(themes[0]);

  useEffect(() => {
    if (localStorage.getItem("theme") !== null) {
      setTheme(localStorage.getItem("theme"));
    } else {
      localStorage.setItem("theme", themes[0]);
    }
  }, [theme]);

  const onChangeLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const lang_code = e.target.value;
    i18n.changeLanguage(lang_code);
  };

  const onChangeLTheme = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newTheme = e.target.value;
    localStorage.setItem("theme", newTheme);
    setTheme(newTheme);
  };

  console.log("theme---------------->" + theme);
  console.log("localstorage theme---------------->" + localStorage.getItem("theme"));

  return (
    <nav className={`theme-${theme}`}>
      <p>
        <Trans i18nKey="title" className="text-primary">
          Welcome to react using <code>react-i18next</code> fully type-safe
        </Trans>
      </p>
      <div>
        <NavLink className={isActive + " bg-primary px-20"} to="/">
          {t("home")}
        </NavLink>
        <NavLink className={isActive + " bg-secondary px-20"} to="/settings">
          {t("settings")}
        </NavLink>
      </div>

      <div>
        <select className="bg-gray-300" defaultValue={i18n.language} onChange={onChangeLang}>
          {LANGUAGES.map(({ code, label }) => (
            <option key={code} value={code}>
              {label}
            </option>
          ))}
        </select>
      </div>

      <div>
        <select className="bg-gray-300" defaultValue={themes[0]} onChange={onChangeLTheme}>
          {themes.map((t) => (
            <option key={t} value={t}>
              {t}
            </option>
          ))}
        </select>
      </div>
    </nav>
  );
};
