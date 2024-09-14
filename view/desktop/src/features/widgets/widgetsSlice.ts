import { nanoid, PayloadAction, createSlice, Slice } from "@reduxjs/toolkit";
import { RootState } from "../../app/store";
import { PagesComps } from "./Lumino";

/**
 * Types of Widgets for this counter example
 */

export interface AppWidget {
  type: PagesComps;
  id: string;
  tabTitle: string;
  active: boolean;
}
/**
 * State that holds the widget information
 */
export interface WidgetsState {
  widgets: AppWidget[];
}

/**
 * Draw one Watcher initially
 */
const initialState: WidgetsState = {
  widgets: [{ type: "HomePage", id: nanoid(), tabTitle: "Home", active: true }],
};

/**
 * create a slice for handling basic widget actions: add, delete, activate
 */
export const widgetsSlice: Slice = createSlice({
  name: "widgets",
  initialState,
  reducers: {
    addWidget: (state, action: PayloadAction<AppWidget>) => {
      state.widgets.push(action.payload);
    },
    deleteWidget: (state, action: PayloadAction<string>) => {
      state.widgets = state.widgets.filter((w) => w.id !== action.payload);
    },
    activateWidget: (state, action: PayloadAction<string>) => {
      state.widgets = state.widgets.map((w) => {
        w.active = w.id === action.payload;
        return w;
      });
    },
  },
});

// export actions
export const { addWidget, deleteWidget, activateWidget } = widgetsSlice.actions;

export const addHomePage = () =>
  addWidget({
    id: nanoid(),
    active: true,
    tabTitle: "Home",
    type: "HomePage",
  });

export const addSettingsPage = () =>
  addWidget({
    id: nanoid(),
    active: true,
    tabTitle: "Settings",
    type: "SettingsPage",
  });

export const addLogsPage = () =>
  addWidget({
    id: nanoid(),
    active: true,
    tabTitle: "Logs",
    type: "LogsPage",
  });

/**
 * selector for the widgets
 */
export const selectWidgets = (state: RootState) => state.widgets.widgets;

export default widgetsSlice.reducer;
