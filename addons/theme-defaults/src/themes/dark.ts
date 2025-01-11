// Default
import { Theme } from "@repo/moss-theme";

import { linearGradient, rgb, rgba } from "../color.ts";

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

    "dv.paneview.active.outline.color": { type: "solid", value: "dodgerblue" },
    "dv.drag.over.background.color": { type: "solid", value: rgba(83, 89, 93, 0.5) },
    "dv.drag.over.border.color": { type: "solid", value: "white" },
    "dv.tabs.container.scrollbar.color": { type: "solid", value: "#888" },
    "dv.icon.hover.background.color": { type: "solid", value: rgba(90, 93, 94, 0.31) },
    "dv.group.view.background.color": { type: "solid", value: "#1E1E1E" },
    "dv.tabs.and.actions.container.background.color": { type: "solid", value: "#252526" },
    "dv.activegroup.visiblepanel.tab.background.color": { type: "solid", value: "#1E1E1E" },
    "dv.activegroup.hiddenpanel.tab.background.color": { type: "solid", value: "#2D2D2D" },
    "dv.inactivegroup.visiblepanel.tab.background.color": { type: "solid", value: "#1E1E1E" },
    "dv.inactivegroup.hiddenpanel.tab.background.color": { type: "solid", value: "#2D2D2D" },
    "dv.tab.divider.color": { type: "solid", value: "#1E1E1E" },
    "dv.activegroup.visiblepanel.tab.color": { type: "solid", value: "white" },
    "dv.activegroup.hiddenpanel.tab.color": { type: "solid", value: "#969696" },
    "dv.inactivegroup.visiblepanel.tab.color": { type: "solid", value: "#8F8F8F" },
    "dv.inactivegroup.hiddenpanel.tab.color": { type: "solid", value: "#626262" },
    "dv.separator.border": { type: "solid", value: rgb(68, 68, 68) },
    "dv.paneview.header.border.color": { type: "solid", value: rgba(204, 204, 204, 0.2) },

    "dv.tabs.and.actions.container.font.size": { type: "solid", value: "13px" },
    "dv.tabs.and.actions.container.height": { type: "solid", value: "35px" },
    "dv.floating.box.shadow": { type: "solid", value: "8px 8px 8px 0px rgba(83, 89, 93, 0.5)" },
    "dv.overlay.z.index": { type: "solid", value: "999" },
  },
};
