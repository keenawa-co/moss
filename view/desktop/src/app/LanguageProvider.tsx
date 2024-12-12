import { useEffect } from "react";

import { useLanguageStore } from "@/store/language";

const LanguageProvider = ({ children }: { children: React.ReactNode }) => {
  const { initializeLanguages } = useLanguageStore();

  useEffect(() => {
    initializeLanguages();
  }, [initializeLanguages]);

  return <>{children}</>;
};

export default LanguageProvider;
