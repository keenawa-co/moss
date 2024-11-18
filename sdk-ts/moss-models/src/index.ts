export interface Colors {
  "primary": string; // prettier-ignore
  "sideBar.background": string;
  "toolBar.background": string;
  "page.background": string;
  "statusBar.background": string;
  "windowsCloseButton.background": string;
  "windowControlsLinux.background": string;
  "windowControlsLinux.text": string;
  "windowControlsLinux.hoverBackground": string;
  "windowControlsLinux.activeBackground": string;
}

// Define the Theme interface with specific properties
export interface Theme {
  name: string;
  slug: string;
  type: string;
  isDefault: boolean;
  colors: Colors;
}
