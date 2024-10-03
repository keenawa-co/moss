import { PayloadAction, Slice, createSelector, createSlice } from "@reduxjs/toolkit";
import * as DesktopComponents from "../../components";
import { RootState } from "..";
type DesktopComponentKeys = keyof typeof DesktopComponents;
type OmittedComponents = Omit<
  Record<DesktopComponentKeys, any>,
  "RootLayout" | "SidebarLayout" | "ContentLayout" | "PropertiesLayout"
>;
type DesktopComponentsOmitted = keyof OmittedComponents;
export interface Accordion {
  id: number;
  title: string;
  content: DesktopComponentsOmitted;
  isOpen?: boolean;
  preferredHeight?: number;
}

export interface AccordionState {
  accordion: Accordion[];
  defaultSizes: number[];
}

const initialState: AccordionState = {
  accordion: [
    {
      id: 1,
      title: "Sidebar",
      content: "Sidebar",
      isOpen: false,
    },
    {
      id: 2,
      title: "General",
      content: "SidebarGeneral",
      isOpen: false,
    },
    {
      id: 3,
      title: "Links",
      content: "SidebarLinks",
      isOpen: false,
    },
  ],
  defaultSizes: [35, 35, 35],
};

export const accordionSlice: Slice<AccordionState> = createSlice({
  name: "accordion",
  initialState,
  reducers: {
    setAccordions: (state, action: PayloadAction<Accordion[]>) => {
      state.accordion = action.payload;
    },
    setDefaultSizes: (state, action: PayloadAction<number[]>) => {
      state.defaultSizes = action.payload;
    },
    updateAccordionById: (state, action: PayloadAction<{ id: number; changes: Partial<Accordion> }>) => {
      const { id, changes } = action.payload;
      const accordionIndex = state.accordion.findIndex((item) => item.id === id);
      if (accordionIndex !== -1) {
        state.accordion[accordionIndex] = {
          ...state.accordion[accordionIndex],
          ...changes,
        };
      }
    },
  },
});

export const { setAccordions, setDefaultSizes, updateAccordionById } = accordionSlice.actions;
export default accordionSlice.reducer;
