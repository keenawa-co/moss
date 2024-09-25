// FIXME: Trans component can also be used for translation
import { useTranslation } from "react-i18next";
import { NavLink } from "react-router-dom";

const isActive = ({ isActive }: any) => `link ${isActive ? "active" : ""}`;

export const Menu = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  return (
    <nav>
      <div>
        <NavLink className={isActive + " bg-green-300 px-20 text-[rgba(var(--color-primary))]"} to="/">
          {t("home")}
        </NavLink>
        <NavLink className={isActive + " bg-orange-300 px-20 text-[rgba(var(--color-primary))]"} to="/settings">
          {t("settings")}
        </NavLink>
        <NavLink className={isActive + " bg-yellow-300 px-20 text-[rgba(var(--color-primary))]"} to="/logs">
          logs
        </NavLink>
      </div>
    </nav>
  );
};
