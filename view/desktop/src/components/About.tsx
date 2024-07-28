import { useTranslation } from "react-i18next";

// import { commands } from '../bindings'

export const About = () => {
  const { t } = useTranslation();

  return (
    <main>
      <h1>{t("about")}</h1>
      <span>{t("user", { name: "Jevgenijs 🦇" })}</span>
    </main>
  );
};
