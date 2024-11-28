import "reflect-metadata";
import { type } from "@tauri-apps/plugin-os";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import "@/assets/index.css";
import "@repo/moss-ui/src/fonts.css";
import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";
import { store } from "./store";
import { PageLoader } from "./components/PageLoader";

const sharedWorker = new SharedWorker("./shared-worker.js");

sharedWorker.port.onmessage = (event) => {
  const { action, data } = event.data;

  if (action === "result") {
    console.log("Result:", data);
  }
};

sharedWorker.port.start();

export function callServiceMethod(methodName: string, args: any[] = []) {
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

const App = lazy(() => import("@/app")); // lazy load the main App component
const rootElement = document.getElementById("root") as HTMLElement; // cache the root element reference

if (rootElement) {
  createRoot(rootElement).render(
    <StrictMode>
      <Provider store={store}>
        <Suspense fallback={<PageLoader />}>
          <App />
        </Suspense>
      </Provider>
    </StrictMode>
  );
}
