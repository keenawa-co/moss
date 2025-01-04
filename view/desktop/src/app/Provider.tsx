import { ReactNode, useEffect } from "react";

import { getState } from "@/api/appearance";
import { useLanguageStore } from "@/store/language";
import { useThemeStore } from "@/store/theme";
import { QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const ENABLE_REACT_QUERY_DEVTOOLS = true;

interface ProviderProps {
  children: ReactNode;
}

const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (err, query) => {
      console.log("Query client error", { err, query });
    },
  }),
  defaultOptions: {
    queries: {
      retry: false,
      networkMode: "always",
      refetchOnWindowFocus: true,
      refetchOnReconnect: false,
      refetchOnMount: false,
    },
  },
});

const useInitializeAppState = () => {
  const { setCurrentTheme } = useThemeStore();
  const { setLanguageCode } = useLanguageStore();

  useEffect(() => {
    const fetchAndSetAppState = async () => {
      try {
        const { preferences, defaults } = await getState();

        const theme = preferences?.theme ?? defaults.theme;
        const localeCode = preferences?.locale?.code ?? defaults.locale.code;

        setCurrentTheme(theme);
        setLanguageCode(localeCode);
      } catch (error) {
        console.error("Failed to fetch app state from backend:", error);
      }
    };

    fetchAndSetAppState();
  }, [setCurrentTheme, setLanguageCode]);
};

const Provider = ({ children }: ProviderProps) => {
  useInitializeAppState();

  return (
    <QueryClientProvider client={queryClient}>
      {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-right" />}
      <LanguageProvider>
        <ThemeProvider>{children}</ThemeProvider>
      </LanguageProvider>
    </QueryClientProvider>
  );
};

export default Provider;
