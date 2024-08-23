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
        <h3>Select language:</h3>
        <LanguageSelector />
      </div>
      <div>
        <h3>Select theme:</h3>
        <ThemeSelector themes={themes} />
      </div>
      <span className="text-primary">{t("description.part1")}</span>
      <span className="text-primary">{t("description.part1", { ns: "ns2" })}</span>
    </main>
  );
};

export default Settings;
