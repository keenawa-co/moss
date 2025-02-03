import { create } from "zustand";

import { SerializedDockview } from "@repo/moss-tabs";

interface TabbedPaneState {
  gridState: SerializedDockview;
  setGridState: (state: SerializedDockview) => void;
}

export const useTabbedPaneStore = create<TabbedPaneState>((set) => ({
  gridState: {
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
  } as SerializedDockview,
  setGridState: (state: SerializedDockview) => set({ gridState: state }),
}));
