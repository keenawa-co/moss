import "reflect-metadata";
import { type } from "@tauri-apps/plugin-os";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import "@/assets/index.css";
import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";
import { store } from "./store";

const osType = type();
if (osType !== "macos") {
  await getCurrentWebviewWindow().setDecorations(false);
}

const App: React.FC = lazy(() => import("@/App")); // lazy load the main App component
const rootElement = document.getElementById("root") as HTMLElement; // cache the root element reference

if (rootElement) {
  createRoot(rootElement).render(
    <StrictMode>
      <Provider store={store}>
        <Suspense fallback={<div>Loading...</div>}>
          <App />
        </Suspense>
      </Provider>
    </StrictMode>
  );
}
