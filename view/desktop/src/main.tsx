import "reflect-metadata";
// import App from "@/App";
import "@/assets/index.css";
import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";
import { store } from "./store";

// createRoot(document.getElementById("root") as HTMLElement).render(
//   <StrictMode>
//     <Provider store={store}>
//       <App />
//     </Provider>
//   </StrictMode>
// );

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
