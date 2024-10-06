import { PayloadAction, Slice, createSelector, createSlice } from "@reduxjs/toolkit";
import * as DesktopComponents from "../../components";
import { RootState } from "..";
type DesktopComponentKeys = keyof typeof DesktopComponents;
type OmittedComponents = Omit<
  Record<DesktopComponentKeys, any>,
  "RootLayout" | "SidebarLayout" | "ContentLayout" | "PropertiesLayout"
>;
type DesktopComponentsOmitted = keyof OmittedComponents;
export interface IAccordion {
  id: number;
  title: string;
  content: DesktopComponentsOmitted;
  isOpen?: boolean;
}

export interface AccordionState {
  accordion: IAccordion[];
  preferredSizes: {
    [key: number]: number;
  };
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
  preferredSizes: {},
};

export const accordionSlice: Slice<AccordionState> = createSlice({
  name: "accordion",
  initialState,
  reducers: {
    setAccordions: (state, action: PayloadAction<IAccordion[]>) => {
      state.accordion = action.payload;
    },
    setPreferredSize: (state, action: PayloadAction<{ id: number; size: number }>) => {
      const { id, size } = action.payload;
      const updatedPreferredSizes = { ...state.preferredSizes, [id]: size };
      state.preferredSizes = updatedPreferredSizes;
    },
    updateAccordionById: (state, action: PayloadAction<{ id: number; changes: Partial<IAccordion> }>) => {
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

export const { setAccordions, setPreferredSize, updateAccordionById } = accordionSlice.actions;
export default accordionSlice.reducer;
