import { useEffect, useState } from "react";
import { initializeLanguage } from "@/atoms/langAtom";

export const usePrepareWindow = () => {
  const [isPreparing, setIsPreparing] = useState(true);

  useEffect(() => {
    initializeLanguage();

    setIsPreparing(false);
  }, []);

  return { isPreparing };
};
