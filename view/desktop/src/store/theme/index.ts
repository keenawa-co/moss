import { create } from "zustand";

import { ThemeDescriptor } from "@repo/desktop-models";

export interface ThemeStore {
  currentTheme: ThemeDescriptor | null;
  setCurrentTheme: (theme: ThemeDescriptor) => void;
}

export const useThemeStore = create<ThemeStore>((set) => ({
  currentTheme: null,
  setCurrentTheme: (theme) => {
    set({ currentTheme: theme });
  },
}));
