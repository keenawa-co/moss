import { create } from "zustand";

export interface ActivityBarStore {
  position: "top" | "bottom" | "left" | "right";
  setPosition: (position: ActivityBarStore["position"]) => void;
}

export const useActivityBarStore = create<ActivityBarStore>((set) => ({
  position: "top",
  setPosition: (position: ActivityBarStore["position"]) => set({ position }),
}));
