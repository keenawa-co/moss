import { configureStore } from "@reduxjs/toolkit";
import { languagesReducer } from "./languages";
import { themesReducer } from "./themes";
import { useDispatch } from "react-redux";
import { accordionReducer } from "./accordion";
import { sidebarReducer } from "./sidebar";

export const store = configureStore({
  reducer: {
    languages: languagesReducer,
    themes: themesReducer,
    accordion: accordionReducer,
    sidebar: sidebarReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch: () => AppDispatch = useDispatch;
