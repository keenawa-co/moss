import React, { useState, useEffect } from "react";

interface ThemeSelectorProps {
  themes: string[];
}

export const ThemeSelector: React.FC<ThemeSelectorProps> = ({ themes }) => {
  const [selectedTheme, setSelectedTheme] = useState<string>(() => {
    return localStorage.getItem("theme") || "";
  });

  useEffect(() => {
    if (selectedTheme) {
      localStorage.setItem("theme", selectedTheme);
      dispatchEvent(new Event("storage"));
    }
  }, [selectedTheme]);

  const handleThemeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedTheme(e.target.value);
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
