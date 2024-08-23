import { useTranslation } from "react-i18next";
import { LanguageSelector, ThemeSelector } from "@/components";
interface SettingsProps {
  themes: string[];
}

export const Settings: React.FC<SettingsProps> = ({ themes }) => {
  const { t } = useTranslation(["ns1", "ns2"]);

  return (
    <main>
      <div>
        <h3>{t("selectLanguage")}</h3>
        <LanguageSelector />
      </div>
      <div>
        <h3>{t("selectTheme")}</h3>
        <ThemeSelector themes={themes} />
      </div>
    </main>
  );
};

export default Settings;
