export interface PrimarySideBarSlice {
  width: number;
  visible: boolean;
  setHeight: (newWidth: number) => void;
  setVisibility: (visibility: boolean) => void;
}

export const useBottomPaneSlice = (
  set: (fn: (state: PrimarySideBarSlice) => PrimarySideBarSlice) => void
): PrimarySideBarSlice => ({
  width: 255,
  visible: true,
  setHeight: (newHeight) =>
    set((state) => ({
      ...state,
      width: newHeight,
      visible: newHeight > 0,
    })),
  setVisibility: (visible) =>
    set((state) => ({
      ...state,
      visible,
    })),
});
