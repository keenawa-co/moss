import { keyAtomFamily } from "@/lib/context";
import { useAtom } from "jotai";
import { PrimitiveAtom } from "jotai";
import { useEffect } from "react";

export function useKeyValue<T = any>(key: string, initialValue?: T): [T | undefined, (value: T) => void] {
  const keyAtom = keyAtomFamily(key) as PrimitiveAtom<T>;

  const [value, setValue] = useAtom(keyAtom);

  useEffect(() => {
    if (value === undefined && initialValue !== undefined) {
      setValue(initialValue);
    }
  }, [value, initialValue]);

  return [value, setValue];
}
