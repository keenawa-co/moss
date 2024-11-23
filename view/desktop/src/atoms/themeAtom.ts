import { ThemeDescriptor } from "@/api/theme";
import { atom } from "jotai";

export const themeAtom = atom<ThemeDescriptor | null>(null);
