import { ThemeDescriptor } from "@/api/appearance";
import { atom } from "jotai";

export const themeAtom = atom<ThemeDescriptor | null>(null);
