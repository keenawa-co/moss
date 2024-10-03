import { createSelector } from "@reduxjs/toolkit";
import { RootState } from "../store"; // Adjust the path to your store
import { Accordion } from "./accordionSlice";

export const selectAccordionById = (id: number) =>
  createSelector(
    (state: RootState) => state.accordion.accordion,
    (accordion) => accordion.find((a: Accordion) => a.id === id)
  );
