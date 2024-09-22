import { PayloadAction, Slice, createSlice } from "@reduxjs/toolkit";

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
