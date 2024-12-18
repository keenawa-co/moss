// Default
import { Theme } from "@repo/moss-desktop";

import { rgba } from "../color.ts";

export const defaultDarkTheme: Theme = {
  name: "Moss Dark Default",
  slug: "moss-dark",
  type: "dark",
  isDefault: false,
  colors: {
    "primary": { "type": "solid", "value": rgba(255, 255, 255, 1) }, // prettier-ignore
    "sideBar.background": { type: "solid", value: rgba(39, 39, 42, 1) },
    "toolBar.background": { type: "solid", value: rgba(30, 32, 33, 1) },
    "page.background": { type: "solid", value: rgba(22, 24, 25, 1) },
    "statusBar.background": { type: "solid", value: "#0F62FE" },
    "windowsCloseButton.background": { type: "solid", value: rgba(196, 43, 28, 1) },
    "windowControlsLinux.background": { type: "solid", value: rgba(55, 55, 55, 1) },
    "windowControlsLinux.text": { type: "solid", value: rgba(255, 255, 255, 1) },
    "windowControlsLinux.hoverBackground": { type: "solid", value: rgba(66, 66, 66, 1) },
    "windowControlsLinux.activeBackground": { type: "solid", value: rgba(86, 86, 86, 1) },
  },
};
