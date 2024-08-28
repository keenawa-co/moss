import { useTranslation } from "react-i18next";
import { LanguageSelector, ThemeSelector } from "@/components";

export const Logs: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  return (
    <main>
      <div>
        <h3>{t("selectLanguage")}</h3>
        <LanguageSelector />
      </div>
      <div>
        <h3>{t("selectTheme")}</h3>
        <ThemeSelector />
      </div>
    </main>
  );
};

export default Logs;
