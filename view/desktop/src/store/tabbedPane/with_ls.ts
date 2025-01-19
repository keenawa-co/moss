import { create } from "zustand";

import { SerializedDockview } from "@repo/moss-tabs";

interface TabbedPaneState {
  gridState: SerializedDockview;
  setGridState: (state: SerializedDockview) => void;
}

// Load state from localStorage
const loadState = (): SerializedDockview => {
  const state = localStorage.getItem("dv-demo-state");
  return state
    ? JSON.parse(state)
    : {
        grid: {
          root: {
            type: "branch",
            data: [],
          },
          height: 0,
          width: 0,
          orientation: "horizontal" as SerializedDockview["grid"]["orientation"],
        },
        panels: {},
        activeGroup: undefined,
        floatingGroups: [],
        popoutGroups: [],
      };
};

// Save state to localStorage
const saveState = (state: SerializedDockview) => {
  localStorage.setItem("dv-demo-state", JSON.stringify(state));
};

export const useTabbedPaneStore = create<TabbedPaneState>((set) => ({
  gridState: loadState(),
  setGridState: (state: SerializedDockview) => {
    set({ gridState: state });
    saveState(state);
  },
}));
