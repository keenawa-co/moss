import "@repo/moss-ui/src/fonts.css";
import "@/assets/index.css";

import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";

import { QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { type } from "@tauri-apps/plugin-os";

import GeneralProvider from "./app/Provider";
import { PageLoader } from "./components/PageLoader";
import { store } from "./store";

const sharedWorker = new SharedWorker("./shared-worker.js");

sharedWorker.port.onmessage = (event) => {
  const { action, data } = event.data;

  if (action === "result") {
    console.log("Result:", data);
  }
};

sharedWorker.port.start();

export function callServiceMethod(methodName: string, args: unknown[] = []) {
  sharedWorker.port.postMessage({
    action: "callMethod",
    data: {
      method: methodName,
      args: args,
    },
  });
}

const osType = type();
if (osType !== "macos") {
  await getCurrentWebviewWindow().setDecorations(false);
}
const ENABLE_REACT_QUERY_DEVTOOLS = true;

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
      refetchOnWindowFocus: false,
      refetchOnReconnect: false,
      refetchOnMount: false,
    },
  },
});

const App = lazy(() => import("@/app")); // lazy load the main App component
const rootElement = document.getElementById("root") as HTMLElement; // cache the root element reference

if (rootElement) {
  createRoot(rootElement).render(
    <StrictMode>
      <Provider store={store}>
        <QueryClientProvider client={queryClient}>
          {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-right" />}
          <GeneralProvider>
            <Suspense fallback={<PageLoader />}>
              <App />
            </Suspense>
          </GeneralProvider>
        </QueryClientProvider>
      </Provider>
    </StrictMode>
  );
}
