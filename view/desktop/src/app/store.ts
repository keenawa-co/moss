import { configureStore, ThunkAction, Action } from "@reduxjs/toolkit";
import widgetsReducer from "../features/widgets/widgetsSlice";
import { useDispatch } from "react-redux";
import { Store } from "redux";

export const store: Store = configureStore({
  reducer: {
    widgets: widgetsReducer,
  },
});

export const useAppDispatch = () => useDispatch<typeof store.dispatch>();

export type RootState = ReturnType<typeof store.getState>;
export type AppThunk<ReturnType = void> = ThunkAction<ReturnType, RootState, unknown, Action<string>>;
