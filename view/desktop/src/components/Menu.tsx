// FIXME: Trans component can also be used for translation
import { Trans, useTranslation } from "react-i18next";
import { NavLink } from "react-router-dom";

const isActive = ({ isActive }: any) => `link ${isActive ? "active" : ""}`;

export const Menu = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  return (
    <nav>
      <p>
        <Trans i18nKey="title" className="text-primary">
          Welcome to react using <code>react-i18next</code> fully type-safe
        </Trans>
      </p>
      <div>
        <NavLink className={isActive + " bg-green-300 text-primary px-20"} to="/">
          {t("home")}
        </NavLink>
        <NavLink className={isActive + " bg-orange-300 text-primary px-20"} to="/settings">
          {t("settings")}
        </NavLink>
      </div>
    </nav>
  );
};
