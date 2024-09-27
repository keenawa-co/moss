import { createAsyncThunk } from "@reduxjs/toolkit";
import { commands } from "@/bindings";
import { Convert } from "@repo/moss-theme";
import { setSelectedTheme, setThemes } from "./themesSlice";
import applyTheme from "../../../../shared/ui/src/tailwind/applyTheme";
import { handleReadTheme } from "./themesHelpers";

export const fetchAllThemes = createAsyncThunk("themes/fetchAllThemes", async (_, { dispatch, rejectWithValue }) => {
  try {
    const response = await commands.fetchAllThemes();
    if (response.status === "error") throw new Error("Failed to fetch themes");

    dispatch(setThemes(response.data));
  } catch (error) {
    if (error instanceof Error) return rejectWithValue(error.message);
  }
});

export const setTheme = createAsyncThunk(
  "themes/setTheme",
  async (themeCode: string, { dispatch, rejectWithValue }) => {
    try {
      const response = await commands.readTheme(themeCode);
      if (response.status === "error") throw new Error("Failed to read theme");

      applyTheme(Convert.toTheme(response.data));
      dispatch(setSelectedTheme(themeCode));
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
