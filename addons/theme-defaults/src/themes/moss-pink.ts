import { Theme } from "@repo/moss-theme";

import { linearGradient, rgba } from "../color.ts";

export const pinkTheme: Theme = {
  name: "Moss Pink",
  slug: "moss-pink",
  type: "light",
  isDefault: false,
  colors: {
    "primary": { "type": "solid", "value": rgba(0, 0, 0, 1) }, // prettier-ignore
    "sideBar.background": { type: "solid", value: rgba(234, 157, 242, 1) },
    "toolBar.background": { type: "solid", value: rgba(222, 125, 232, 1) },
    "page.background": { type: "solid", value: rgba(227, 54, 245, 1) },
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
    "windowControlsLinux.background": { type: "solid", value: rgba(218, 218, 218, 1) },
    "windowControlsLinux.text": { type: "solid", value: rgba(61, 61, 61, 1) },
    "windowControlsLinux.hoverBackground": { type: "solid", value: rgba(209, 209, 209, 1) },
    "windowControlsLinux.activeBackground": { type: "solid", value: rgba(191, 191, 191, 1) },
  },
};
