import { createSelector } from "@reduxjs/toolkit";
import { RootState } from "../index";
import { IAccordion } from "./accordionSlice";

export const selectAccordionById = (id: number) =>
  createSelector(
    (state: RootState) => state.accordion.accordion,
    (accordion) => accordion.find((a: IAccordion) => a.id === id)
  );

export const getPreferredSizeById = (id: number) =>
  createSelector(
    (state: RootState) => state.accordion.preferredSizes,
    (preferredSizes) => preferredSizes[id]
  );
