import { configureStore } from "@reduxjs/toolkit";
import languagesReducer from "./languages/languagesSlice";
import themesReducer from "./themes/themesSlice";
import { useDispatch } from "react-redux";

export const store = configureStore({
  reducer: {
    languages: languagesReducer,
    themes: themesReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch: () => AppDispatch = useDispatch;
