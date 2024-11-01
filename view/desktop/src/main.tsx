import App from "@/App";
import "@/assets/index.css";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";
import { store } from "./store";
import { type } from "@tauri-apps/plugin-os";

if (type() !== "windows") {
  document.querySelectorAll("html, body").forEach((el) => {
    el.classList.add("rounded-xl");
  });
}

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <Provider store={store}>
      <App />
    </Provider>
  </StrictMode>
);
