import { Slice, createSlice } from "@reduxjs/toolkit";

export interface SidebarState {
  sidebarVisible: boolean;
}

const initialState: SidebarState = {
  sidebarVisible: true,
};

export const sidebarSlice: Slice<SidebarState> = createSlice({
  name: "sidebar",
  initialState,
  reducers: {
    toggleSidebarVisibility: (state) => {
      state.sidebarVisible = !state.sidebarVisible;
    },
  },
});

export const { toggleSidebarVisibility } = sidebarSlice.actions;
export default sidebarSlice.reducer;
