import { configureStore } from "@reduxjs/toolkit";
import languagesReducer from "./languages/languagesSlice";
import themesReducer from "./themes/themesSlice";
import { useDispatch } from "react-redux";
import { accordionReducer } from "./accordion";

export const store = configureStore({
  reducer: {
    languages: languagesReducer,
    themes: themesReducer,
    accordion: accordionReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch: () => AppDispatch = useDispatch;
