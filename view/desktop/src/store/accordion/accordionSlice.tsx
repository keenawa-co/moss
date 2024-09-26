import { PayloadAction, Slice, createSlice } from "@reduxjs/toolkit";

export interface Accordion {
  title: string;
  content: string;
  isOpen?: boolean;
}

export interface AccordionState {
  accordion: Accordion[];
}

const initialState: AccordionState = {
  accordion: [
    {
      title: "Sidebar",
      content: "Sidebar",
      isOpen: false,
    },
    {
      title: "Accordion 2",
      content: "Accordion 2",
      isOpen: false,
    },
    {
      title: "Accordion 3",
      content: "Accordion 3",
      isOpen: false,
    },
  ],
};

export const accordionSlice: Slice<AccordionState> = createSlice({
  name: "accordion",
  initialState,
  reducers: {
    setAccordion: (state, action: PayloadAction<Accordion[]>) => {
      console.log("setAccordion", action.payload);
      state.accordion = action.payload;
    },
  },
});

export const { setAccordion } = accordionSlice.actions;
export default accordionSlice.reducer;
