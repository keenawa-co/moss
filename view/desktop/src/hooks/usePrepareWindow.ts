import { useEffect, useState } from "react";

import { useLanguageStore } from "@/store/language";

export const usePrepareWindow = () => {
  const [isPreparing, setIsPreparing] = useState(true);
  const { initializeLanguage } = useLanguageStore();

  useEffect(() => {
    initializeLanguage();
    setIsPreparing(false);
  }, [initializeLanguage]);

  return { isPreparing };
};
