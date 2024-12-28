import { useTranslation } from "react-i18next";

import { LanguageSelector } from "@/components";
import { DockviewReact, DockviewReadyEvent, IDockviewPanelProps } from "@repo/moss-tabs";

import ThemeSwitcher from "../components/ThemeSwitcher";

import "@repo/moss-tabs/styles/dockview.css";

export const Settings = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  return (
    <main>
      <div>
        <h3>{t("selectLanguage")}</h3>
        <LanguageSelector />
      </div>
      <div>
        <h3>{t("selectTheme")}</h3>
        <ThemeSwitcher />
      </div>
    </main>
  );
};
