import { create } from "zustand";

import { getColorThemes } from "@/api/appearance";
import { ThemeDescriptor } from "@repo/moss-desktop";

export interface ThemeStore {
  currentTheme: ThemeDescriptor | null;
  setCurrentTheme: (theme: ThemeDescriptor) => void;
}

export const useThemeStore = create<ThemeStore>((set) => ({
  currentTheme: null,
  setCurrentTheme: async (theme) => {
    const res = await getColorThemes();
    console.log(res);
    set({ currentTheme: theme });
  },
}));
