/* eslint-disable @typescript-eslint/naming-convention */

export interface IThemeRGB {
  "rgb-primary"?: string;
  "rgb-sidebar-background"?: string;
  "rgb-toolbar-background"?: string;
  "rgb-page-background"?: string;
  "rgb-statusbar-background"?: string;

  "rgb-windows-close-button-background"?: string;

  "rgb-window-controls-linux-background"?: string;
  "rgb-window-controls-linux-text"?: string;
  "rgb-window-controls-linux-background-hover"?: string;
  "rgb-window-controls-linux-background-active"?: string;
}

export interface IThemeVariables {
  "--color-primary": string;
  "--color-sidebar-background": string;
  "--color-toolbar-background": string;
  "--color-page-background": string;
  "--color-statusbar-background": string;

  "--color-windows-close-button-background": string;

  "--color-window-controls-linux-background": string;
  "--color-window-controls-linux-text": string;
  "--color-window-controls-linux-background-hover": string;
  "--color-window-controls-linux-background-active": string;
}

export interface IThemeColors {
  primary?: string;
  "sidebar-background"?: string;
  "toolbar-background"?: string;
  "page-background"?: string;
  "statusbar-background"?: string;

  "windows-close-button-background"?: string;

  "window-controls-linux-background"?: string;
  "window-controls-linux-text"?: string;
  "window-controls-linux-background-hover"?: string;
  "window-controls-linux-background-active"?: string;
}
