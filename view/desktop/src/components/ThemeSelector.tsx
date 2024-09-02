import React, { useState, useEffect } from "react";

interface ThemeSelectorProps {
  themes: string[];
}

export const ThemeSelector: React.FC<ThemeSelectorProps> = ({ themes }) => {
  const [theme, setTheme] = useState<string>(() => {
    const savedTheme = localStorage.getItem("theme");
    return savedTheme && themes.includes(savedTheme) ? savedTheme : themes[0];
  });

  useEffect(() => {
    localStorage.setItem("theme", theme);
    dispatchEvent(new Event("storage"));
  }, [theme]);

  const onChangeTheme = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newTheme = e.target.value;
    setTheme(newTheme);
  };

  return (
    <div>
      <select className="bg-pink-300 text-primary" defaultValue={theme} onChange={onChangeTheme}>
        {themes.map((t) => (
          <option key={t} value={t}>
            {t.charAt(0).toUpperCase() + t.slice(1)}
          </option>
        ))}
      </select>
    </div>
  );
};

export default ThemeSelector;
