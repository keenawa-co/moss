import { PayloadAction, Slice, createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { commands } from "@/bindings";
import { Convert } from "@repo/theme";
import applyTheme from "../../../../shared/ui/src/tailwind/applyTheme";

export const handleReadTheme = async (themeName: string) => {
  try {
    let response = await commands.readTheme(themeName);

    if (response.status === "error") throw new Error("Failed to read theme: Invalid response status");

    return Convert.toTheme(response.data);
  } catch (error) {
    if (error instanceof Error) console.error(error);
  }
};

export const setTheme = createAsyncThunk(
  "themes/setTheme",
  async (themeCode: string, { dispatch, rejectWithValue }) => {
    try {
      const response = await commands.readTheme(themeCode);
      if (response.status === "error") {
        throw new Error("Failed to read theme: Invalid response status");
      }

      applyTheme(Convert.toTheme(response.data));
      dispatch(setSelectedTheme(themeCode));
    } catch (error) {
      if (error instanceof Error) return rejectWithValue(error.message);
    }
  }
);

export const fetchAllThemes = createAsyncThunk("themes/fetchAllThemes", async (_, { dispatch, rejectWithValue }) => {
  try {
    const response = await commands.fetchAllThemes();

    if (response.status === "error") {
      throw new Error("Failed to fetch themes: Invalid response status");
    }

    dispatch(setThemes(response.data));
  } catch (error) {
    if (error instanceof Error) return rejectWithValue(error.message);
  }
});

export const setThemeFromLocalStorage = createAsyncThunk(
  "themes/setThemeFromLocalStorage",
  async (_, { dispatch, rejectWithValue }) => {
    try {
      const savedThemeName = localStorage.getItem("theme");
      if (!savedThemeName) {
        dispatch(setSelectedTheme("moss-light"));
        return;
      }

      const themeToUse = await handleReadTheme(savedThemeName);
      if (!themeToUse) {
        dispatch(setSelectedTheme("moss-light"));
        return;
      }

      applyTheme(themeToUse);
      dispatch(setSelectedTheme(themeToUse.name));
    } catch (error) {
      console.error("Failed to initialize themes:", error);
      return rejectWithValue(error);
    }
  }
);

export const initializeThemes = createAsyncThunk("themes/initializeThemes", async (_, { dispatch }) => {
  dispatch(fetchAllThemes());
  dispatch(setThemeFromLocalStorage());
});

export interface ThemesState {
  themes: string[];
  selected: string | undefined;
}

const initialState: ThemesState = {
  themes: [],
  selected: undefined,
};

export const themesSlice: Slice<ThemesState> = createSlice({
  name: "themes",
  initialState,
  reducers: {
    setSelectedTheme: (state, action: PayloadAction<string>) => {
      if (action.payload === state.selected) return;

      state.selected = action.payload;
      localStorage.setItem("theme", action.payload);
    },
    setThemes: (state, action: PayloadAction<string[]>) => {
      state.themes = action.payload;
    },
  },
});

export const { setSelectedTheme, setThemes } = themesSlice.actions;
export default themesSlice.reducer;
