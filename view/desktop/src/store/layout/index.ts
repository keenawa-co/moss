import { create } from "zustand";

//TODO this type should be imported from backend in the future
export type LayoutAlignment = "center" | "justify" | "left" | "right";
export type LayoutPrimarySideBarPosition = "left" | "right";
export interface LayoutState {
  alignment: LayoutAlignment;
  primarySideBarPosition: "left" | "right";
  setPrimarySideBarPosition: (position: LayoutPrimarySideBarPosition) => void;
  setAlignment: (alignment: LayoutState["alignment"]) => void;
  primarySideBar: {
    width: number;
    visibility: boolean;
    setWidth: (newWidth: number) => void;
    setVisibility: (visibility: boolean) => void;
    getWidth: () => number;
  };
  secondarySideBar: {
    width: number;
    visibility: boolean;
    setWidth: (newWidth: number) => void;
    setVisibility: (visibility: boolean) => void;
    getWidth: () => number;
  };
  bottomPane: {
    height: number;
    visibility: boolean;
    setHeight: (newHeight: number) => void;
    setVisibility: (visibility: boolean) => void;
    getHeight: () => number;
  };
}

export const useLayoutStore = create<LayoutState>()((set, get) => ({
  alignment: "center",
  primarySideBarPosition: "left",
  setPrimarySideBarPosition: (position: LayoutPrimarySideBarPosition) => {
    set({ primarySideBarPosition: position });
  },
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
    getWidth: () => {
      return get().primarySideBar.width;
    },
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
    getWidth: () => {
      return get().secondarySideBar.width;
    },
  },
  bottomPane: {
    height: 333,
    visibility: true,
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
    getHeight: () => {
      return get().bottomPane.height;
    },
  },
}));
