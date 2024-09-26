import { Sidebar } from "@/components";
import { PayloadAction, Slice, createSlice } from "@reduxjs/toolkit";
import { ReactElement } from "react";

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
      title: "Accordion 1",
      content: "Sidebar", // i want to render this, but error occurs: [Error] ReferenceError: Cannot access uninitialized variable. Module Code (accordionSlice.tsx:8)
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
