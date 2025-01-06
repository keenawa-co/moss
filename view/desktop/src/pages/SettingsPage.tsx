import { useTranslation } from "react-i18next";

import { useChangeColorTheme } from "@/hooks/useChangeColorTheme";
import { useChangeLanguagePack } from "@/hooks/useChangeLanguagePack";
import { useGetAppState } from "@/hooks/useGetAppState";
import { useGetColorThemes } from "@/hooks/useGetColorThemes";
import { useGetLanguagePacks } from "@/hooks/useGetLanguagePacks";

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const { data: appState } = useGetAppState();

  const { data: themes } = useGetColorThemes();
  const { mutate: mutateChangeColorTheme } = useChangeColorTheme();

  const { data: langs } = useGetLanguagePacks();
  const { mutate: mutateChangeLanguagePack } = useChangeLanguagePack();

  const handleLanguageChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedCode = event.target.value;
    const selectedLang = langs?.find((lang) => lang.code === selectedCode);
    if (selectedLang) {
      mutateChangeLanguagePack(selectedLang);
    }
  };

  const handleThemeChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedId = event.target.value;
    const selectedTheme = themes?.find((theme) => theme.id === selectedId);
    if (selectedTheme) {
      mutateChangeColorTheme(selectedTheme);
    }
  };

  console.log("settings page render");

  return (
    <main>
      <div>
        <h3>{t("selectLanguage")}</h3>
        <select
          id="lang-select"
          className="rounded border p-2"
          value={appState?.preferences.locale?.code || appState?.defaults.locale?.code}
          onChange={handleLanguageChange}
        >
          {langs?.map((lang) => (
            <option key={lang.code} value={lang.code}>
              {lang.name}
            </option>
          ))}
        </select>
      </div>

      <div>
        <h3>{t("selectTheme")}</h3>
        <select
          id="theme-select"
          className="rounded border p-2"
          value={appState?.preferences.theme?.id || appState?.defaults.theme?.id}
          onChange={handleThemeChange}
        >
          {themes?.map((theme) => (
            <option key={theme.id} value={theme.id}>
              {theme.name}
            </option>
          ))}
        </select>
      </div>
    </main>
  );
};
