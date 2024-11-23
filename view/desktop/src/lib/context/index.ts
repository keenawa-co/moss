import { atomFamily } from "jotai/utils";
import { atom, PrimitiveAtom } from "jotai";

export const keyAtomFamily = atomFamily<string, PrimitiveAtom<any>>((key) => atom(undefined));
