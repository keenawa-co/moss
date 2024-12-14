import { create } from "zustand";

export interface LayoutState {
  primarySideBar: {
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
