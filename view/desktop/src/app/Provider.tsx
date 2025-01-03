import { ReactNode, useEffect, useRef } from "react";

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
  const { setCurrentTheme, themes } = useThemeStore();
  const { setLanguageCode } = useLanguageStore();
  const initialized = useRef(false);

  useEffect(() => {
    if (initialized.current) return;
    initialized.current = true;

    // console.log("useEffect useInitializeAppState");

    const fetchAndSetAppState = async () => {
      // console.log("fetchAndSetAppState");
      try {
        const { preferences } = await getState();
        // console.log("preferences.theme.source", preferences.theme.source);
        // FIXME: This is a temporary solution until preferences.theme.source returns the correct theme source.
        if (preferences.theme.source === "moss-light.css") {
          setCurrentTheme(themes[0]);
        } else if (preferences.theme.source === "moss-dark.css") {
          setCurrentTheme(themes[1]);
        } else {
          setCurrentTheme(preferences.theme);
        }

        setLanguageCode(preferences.locale.code);
      } catch (error) {
        console.error("Failed to fetch app state from backend:", error);
      }
    };

    fetchAndSetAppState();
  }, [setCurrentTheme, setLanguageCode, themes]);
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
