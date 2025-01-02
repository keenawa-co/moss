import { create } from "zustand";

import { ThemeDescriptor } from "@repo/moss-desktop";

export interface ThemeStore {
  currentTheme: ThemeDescriptor | null;
  setCurrentTheme: (theme: ThemeDescriptor) => void;
}

export const useThemeStore = create<ThemeStore>((set) => ({
  currentTheme: null,
  setCurrentTheme: async (theme) => {
    set({ currentTheme: theme });
  },
}));
