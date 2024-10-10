import { PayloadAction, Slice, createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import i18n from "@/i18n";
import { LANGUAGES } from "@/constants";

type LanguageCodes = (typeof LANGUAGES)[number]["code"];

export interface LanguagesState {
  code: LanguageCodes;
}

const initialState: LanguagesState = {
  code: "en",
};

export const setLanguageFromLocalStorage = createAsyncThunk(
  "languages/setLanguageFromLocalStorage",
  async (_, { dispatch }) => {
    let savedLanguage = localStorage.getItem("language") as LanguageCodes | null;

    if (!savedLanguage || !LANGUAGES.some(({ code }) => code === savedLanguage)) {
      savedLanguage = "en";
      localStorage.setItem("language", savedLanguage);
    }

    i18n.changeLanguage(savedLanguage);
    dispatch(setLanguage(savedLanguage));
  }
);

export const languagesSlice: Slice<LanguagesState> = createSlice({
  name: "languages",
  initialState,
  reducers: {
    setLanguage: (state, action: PayloadAction<LanguageCodes>) => {
      const newLanguage = action.payload;

      localStorage.setItem("language", newLanguage);
      i18n.changeLanguage(newLanguage);
      state.code = newLanguage;
    },
  },
});

export const { setLanguage } = languagesSlice.actions;
export default languagesSlice.reducer;
