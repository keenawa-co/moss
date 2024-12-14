import { create } from "zustand";

export interface LayoutState {
  alignment: "center" | "justify" | "left" | "right";
  setAlignment: (alignment: LayoutState["alignment"]) => void;
  primarySideBar: {
    width: number;
    visibility: boolean;
    setWidth: (newWidth: number) => void;
    setVisibility: (visibility: boolean) => void;
  };
  secondarySideBar: {
    width: number;
    visibility: boolean;
    setWidth: (newWidth: number) => void;
    setVisibility: (visibility: boolean) => void;
  };
  bottomPane: {
    height: number;
    visibility: boolean;
    setHeight: (newHeight: number) => void;
    setVisibility: (visibility: boolean) => void;
  };
}

export const useLayoutStore = create<LayoutState>()((set) => ({
  alignment: "center",
  setAlignment: (newAlignment: LayoutState["alignment"]) => set({ alignment: newAlignment }),
  primarySideBar: {
    width: 255,
    visibility: true,
    setWidth: (newWidth) =>
      set((state) => ({
        primarySideBar: {
          ...state.primarySideBar,
          width: newWidth,
          visibility: newWidth > 0,
        },
      })),
    setVisibility: (visibility) =>
      set((state) => ({
        primarySideBar: {
          ...state.primarySideBar,
          visibility,
        },
      })),
  },
  secondarySideBar: {
    width: 255,
    visibility: true,
    setWidth: (newWidth) =>
      set((state) => ({
        secondarySideBar: {
          ...state.secondarySideBar,
          width: newWidth,
          visibility: newWidth > 0,
        },
      })),
    setVisibility: (visibility) =>
      set((state) => ({
        secondarySideBar: {
          ...state.secondarySideBar,
          visibility,
        },
      })),
  },
  bottomPane: {
    height: 333,
    visibility: false,
    setHeight: (newHeight) =>
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          height: newHeight,
          visibility: newHeight > 0,
        },
      })),
    setVisibility: (visibility) =>
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          visibility,
        },
      })),
  },
}));
