import { Theme } from "@repo/moss-theme";

import { rgb, rgba } from "../color.ts";

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

    "dv.paneview.active.outline.color": { type: "solid", value: "dodgerblue" },
    "dv.drag.over.background.color": { type: "solid", value: rgba(83, 89, 93, 0.5) },
    "dv.drag.over.border.color": { type: "solid", value: "white" },
    "dv.tabs.container.scrollbar.color": { type: "solid", value: "#888" },
    "dv.icon.hover.background.color": { type: "solid", value: rgba(90, 93, 94, 0.31) },
    "dv.group.view.background.color": { type: "solid", value: "white" },
    "dv.tabs.and.actions.container.background.color": { type: "solid", value: "#F3F3F3" },
    "dv.activegroup.visiblepanel.tab.background.color": { type: "solid", value: "white" },
    "dv.activegroup.hiddenpanel.tab.background.color": { type: "solid", value: "#ECECEC" },
    "dv.inactivegroup.visiblepanel.tab.background.color": { type: "solid", value: "white" },
    "dv.inactivegroup.hiddenpanel.tab.background.color": { type: "solid", value: "#ECECEC" },
    "dv.tab.divider.color": { type: "solid", value: "white" },
    "dv.activegroup.visiblepanel.tab.color": { type: "solid", value: rgb(51, 51, 51) },
    "dv.activegroup.hiddenpanel.tab.color": { type: "solid", value: rgba(51, 51, 51, 0.7) },
    "dv.inactivegroup.visiblepanel.tab.color": { type: "solid", value: rgba(51, 51, 51, 0.7) },
    "dv.inactivegroup.hiddenpanel.tab.color": { type: "solid", value: rgba(51, 51, 51, 0.35) },
    "dv.separator.border": { type: "solid", value: rgba(128, 128, 128, 0.35) },
    "dv.paneview.header.border.color": { type: "solid", value: rgb(51, 51, 51) },

    "dv.tabs.and.actions.container.font.size": { type: "solid", value: "13px" },
    "dv.tabs.and.actions.container.height": { type: "solid", value: "35px" },
    "dv.floating.box.shadow": { type: "solid", value: "8px 8px 8px 0px rgba(83, 89, 93, 0.5)" },
    "dv.overlay.z.index": { type: "solid", value: "999" },
  },
};
