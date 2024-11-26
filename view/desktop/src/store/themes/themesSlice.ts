// import { PayloadAction, Slice, createSlice } from "@reduxjs/toolkit";

// export interface ThemesState {
//   themes: string[];
//   selected: string | undefined;
//   isThemeSelected: boolean;
// }

// const initialState: ThemesState = {
//   themes: [],
//   selected: undefined,
//   isThemeSelected: false,
// };

// export const themesSlice: Slice<ThemesState> = createSlice({
//   name: "themes",
//   initialState,
//   reducers: {
//     setSelectedTheme: (state, action: PayloadAction<string>) => {
//       const newTheme = action.payload;
//       if (newTheme === state.selected) return;

//       if (!newTheme) {
//         state.isThemeSelected = false;
//       } else {
//         state.isThemeSelected = true;
//       }

//       state.selected = newTheme;
//       localStorage.setItem("theme", newTheme);
//     },
//     setThemes: (state, action: PayloadAction<string[]>) => {
//       state.themes = action.payload;
//     },
//   },
// });

// export const { setSelectedTheme, setThemes } = themesSlice.actions;
// export default themesSlice.reducer;
