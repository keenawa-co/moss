import { ThemeDescriptor } from "@repo/desktop-models";
import { atom } from "jotai";

export const themeAtom = atom<ThemeDescriptor | null>(null);
