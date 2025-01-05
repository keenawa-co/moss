// Default
import { Theme } from "@repo/moss-theme";

import { linearGradient, rgba } from "../color.ts";

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
    "statusBar.background": {
      type: "gradient",
      value: linearGradient(
        "90deg",
        [rgba(255, 0, 0, 1), 0],
        [rgba(224, 0, 41, 1), 16],
        [rgba(190, 0, 87, 1), 34],
        [rgba(155, 0, 133, 1), 51],
        [rgba(63, 0, 255, 1), 100]
      ),
    },
    "windowsCloseButton.background": { type: "solid", value: rgba(196, 43, 28, 1) },
    "windowControlsLinux.background": { type: "solid", value: rgba(55, 55, 55, 1) },
    "windowControlsLinux.text": { type: "solid", value: rgba(255, 255, 255, 1) },
    "windowControlsLinux.hoverBackground": { type: "solid", value: rgba(66, 66, 66, 1) },
    "windowControlsLinux.activeBackground": { type: "solid", value: rgba(86, 86, 86, 1) },
  },
};
