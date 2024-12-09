import { Theme } from "@repo/desktop-models";
import { rgba } from "../color.ts";

export const defaultLightTheme: Theme = {
  name: "Moss Light Default",
  slug: "moss-light",
  type: "light",
  isDefault: true,
  colors: {
    "primary": { "type": "solid", "value": rgba(0, 0, 0, 1) }, // prettier-ignore
    "sideBar.background": { type: "solid", value: rgba(244, 244, 245, 1) },
    "toolBar.background": { type: "solid", value: rgba(224, 224, 224, 1) },
    "page.background": { type: "solid", value: rgba(255, 255, 255, 1) },
    "statusBar.background": { type: "solid", value: "#0F62FE" },
    "windowsCloseButton.background": { type: "solid", value: rgba(196, 43, 28, 1) },
    "windowControlsLinux.background": { type: "solid", value: rgba(218, 218, 218, 1) },
    "windowControlsLinux.text": { type: "solid", value: rgba(61, 61, 61, 1) },
    "windowControlsLinux.hoverBackground": { type: "solid", value: rgba(209, 209, 209, 1) },
    "windowControlsLinux.activeBackground": { type: "solid", value: rgba(191, 191, 191, 1) },
  },
};
