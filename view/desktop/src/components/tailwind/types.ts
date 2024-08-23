/* eslint-disable @typescript-eslint/naming-convention */

export interface IThemeRGB {
  "rgb-primary"?: string;
  "rgb-sidebar-background"?: string;
  "rgb-toolbar-background"?: string;
  "rgb-page-background"?: string;
  "rgb-statusbar-background"?: string;
}

export interface IThemeVariables {
  "--color-primary": string;
  "--color-sidebar-background": string;
  "--color-toolbar-background": string;
  "--color-page-background": string;
  "--color-statusbar-background": string;
}

export interface IThemeColors {
  primary?: string;
  "sidebar-background"?: string;
  "toolbar-background"?: string;
  "page-background"?: string;
  "statusbar-background"?: string;
}
