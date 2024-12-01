import React, { ReactNode } from "react";
import { QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { Provider as JotaiProvider } from "jotai";
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
      refetchOnMount: false, // Don't refetch when a hook mounts
    },
  },
});

const Provider: React.FC<ProviderProps> = ({ children }) => {
  return (
    <QueryClientProvider client={queryClient}>
      {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-left" />}
      <JotaiProvider>
        <ThemeProvider>{children}</ThemeProvider>
      </JotaiProvider>
    </QueryClientProvider>
  );
};

export default Provider;
