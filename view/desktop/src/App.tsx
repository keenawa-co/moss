import { QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { ContentLayout, LaunchPad, Menu, RootLayout } from "@/components";
import "@/i18n";
import { Suspense, useState } from "react";
import { useSelector } from "react-redux";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { Home, Logs, Settings } from "./components/pages";
import { useInitializeApp } from "./hooks/useInitializeApp";
import { RootState } from "./store";
import { callServiceMethod } from "./main";
import { PageLoader } from "./components/PageLoader";
import { useUpdateStoredString } from "./hooks/useReactQuery";

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

const App: React.FC = () => {
  const { isInitializing, initializationError } = useInitializeApp();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);

  callServiceMethod("doSomething");

  if (isInitializing) {
    return <PageLoader />;
  }

  if (initializationError) {
    return (
      <div className="relative flex min-h-screen bg-storm-800">
        <div className="container mx-auto flex max-w-screen-xl flex-col items-center justify-center text-2xl text-red-500">
          <p>Initialization Failed</p>
          <p>{initializationError.message}</p>
        </div>
      </div>
    );
  }

  const ENABLE_REACT_QUERY_DEVTOOLS = true;

  return (
    <QueryClientProvider client={queryClient}>
      {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools buttonPosition="bottom-left" />}
      <RootLayout>
        <Resizable proportionalLayout={false}>
          <ResizablePanel minSize={100} preferredSize={255} snap visible={isSidebarVisible} className="select-none">
            <LaunchPad />
          </ResizablePanel>
          <ResizablePanel>
            <ContentLayout className="content relative flex h-full flex-col overflow-auto">
              <Suspense fallback={<PageLoader />}>
                <BrowserRouter>
                  <Menu />
                  <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/settings" element={<Settings />} />
                    <Route path="/logs" element={<Logs />} />
                  </Routes>
                </BrowserRouter>
              </Suspense>
            </ContentLayout>
          </ResizablePanel>
        </Resizable>
      </RootLayout>
    </QueryClientProvider>
  );
};

export default App;
