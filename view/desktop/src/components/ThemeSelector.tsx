import { useAppDispatch, RootState } from "@/store";
import { setTheme } from "@/store/themes";
import React from "react";
import { useSelector } from "react-redux";

export const ThemeSelector = () => {
  const dispatch = useAppDispatch();

  const selectedTheme = useSelector((state: RootState) => state.themes.selected);
  const themes = useSelector((state: RootState) => state.themes.themes);

  const handleThemeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    dispatch(setTheme(e.target.value));
  };

  return (
    <div>
      <select
        className="bg-pink-300 text-[rgba(var(--color-primary))]"
        value={selectedTheme}
        onChange={handleThemeChange}
      >
        {themes.map((theme) => (
          <option key={theme} value={theme}>
            {theme.charAt(0).toUpperCase() + theme.slice(1)}
          </option>
        ))}
      </select>
    </div>
  );
};

export default ThemeSelector;
