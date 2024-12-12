import { useDispatch } from "react-redux";

import { configureStore } from "@reduxjs/toolkit";

import { accordionReducer } from "./accordion";

export const store = configureStore({
  reducer: {
    accordion: accordionReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch: () => AppDispatch = useDispatch;
