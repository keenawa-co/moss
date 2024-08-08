import type { HTMLProps } from "react";

export interface WindowControlsProps extends HTMLProps<HTMLDivElement> {
  platform?: "windows" | "macos" | "gnome";

  hide?: boolean;

  hideMethod?: "display" | "visibility";

  justify?: boolean;

  "data-tauri-drag-region"?: boolean;
}

export interface WindowTitlebarProps extends HTMLProps<HTMLDivElement> {
  controlsOrder?: "right" | "left" | "platform" | "system";

  windowControlsProps?: WindowControlsProps;
}
