import React from "react";
import { useAtom } from "jotai";
import { themeAtom } from "../atoms/themeAtom";
import { useFetchThemes } from "../hooks/useFetchThemes";
import { useChangeTheme } from "../hooks/useChangeTheme";

const ThemeSwitcher: React.FC = () => {
  const [currentTheme, setCurrentTheme] = useAtom(themeAtom);
  const { data: themes, isLoading, error } = useFetchThemes();
  const { mutate: mutateChangeTheme } = useChangeTheme();

  const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedThemeId = e.target.value;
    const selectedTheme = themes?.find((theme) => theme.id === selectedThemeId);

    if (selectedTheme) {
      mutateChangeTheme(selectedThemeId, {
        onSuccess: () => {
          setCurrentTheme(selectedTheme);
        },
        onError: (error: Error) => {
          console.error("Error changing theme:", error);
        },
      });
    }
  };

  if (isLoading) return <p>Loading themes...</p>;
  if (error) return <p>Error loading themes: {error.message}</p>;

  return (
    <div>
      <select id="theme-select" value={currentTheme?.id || ""} onChange={handleChange} className="rounded border p-2">
        {themes?.map((theme) => (
          <option key={theme.id} value={theme.id}>
            {theme.name}
          </option>
        ))}
      </select>
    </div>
  );
};

export default ThemeSwitcher;
