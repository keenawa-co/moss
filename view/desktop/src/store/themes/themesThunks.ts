import { createAsyncThunk } from "@reduxjs/toolkit";
import { setSelectedTheme, setThemes } from "./themesSlice";
import { invokeIpc } from "@/lib/backend/tauri";
import { applyTheme } from "@repo/moss-theme";
import { handleReadTheme } from "./themesHelpers";
import { Theme } from "@repo/desktop-models";

export const fetchAllThemes = createAsyncThunk("themes/fetchAllThemes", async (_, { dispatch, rejectWithValue }) => {
  try {
    const response = await invokeIpc<string[], string>("fetch_all_themes");
    if (response.status === "error") throw new Error("Failed to fetch themes");

    dispatch(setThemes(response.data));
  } catch (error) {
    if (error instanceof Error) return rejectWithValue(error.message);
  }
});

export const setTheme = createAsyncThunk(
  "themes/setTheme",
  async (themeName: string, { dispatch, rejectWithValue }) => {
    try {
      const response = await invokeIpc<Theme, string>("read_theme", { themeName });
      if (response.status === "error") throw new Error("Failed to read theme");
      const theme: Theme = response.data;
      applyTheme(theme);
      dispatch(setSelectedTheme(themeName));
    } catch (error) {
      if (error instanceof Error) return rejectWithValue(error.message);
    }
  }
);

export const setThemeFromLocalStorage = createAsyncThunk(
  "themes/setThemeFromLocalStorage",
  async (_, { dispatch, rejectWithValue }) => {
    try {
      const savedThemeName = localStorage.getItem("theme");
      const themeToUse = savedThemeName ? await handleReadTheme(savedThemeName) : null;

      if (!themeToUse) {
        dispatch(setSelectedTheme("moss-light"));
      } else {
        applyTheme(themeToUse);
        dispatch(setSelectedTheme(themeToUse.slug || "moss-light"));
      }
    } catch (error) {
      if (error instanceof Error) return rejectWithValue(error);
    }
  }
);

export const initializeThemes = createAsyncThunk("themes/initializeThemes", async (_, { dispatch }) => {
  dispatch(fetchAllThemes());
  dispatch(setThemeFromLocalStorage());
});
