//import { InputTheme } from "@repo/theme";
import { existsSync, mkdirSync, writeFileSync } from "fs";
import * as os from "os";

// FIXME: temporary solution
const homeDirectory = os.homedir();
const themesDirectory = `${homeDirectory}/.config/moss/themes`;

export interface InputTheme {
  name: string;
  type: string;
  default: boolean;
  colors: InputThemeColors;
}

export interface InputThemeColors {
  primary: string;
  "sideBar.background": string;
  "toolBar.background": string;
  "page.background": string;
  "statusBar.background": string;
  "windowsCloseButton.background": string;
  "windowControlsLinux.background": string;
  "windowControlsLinux.text": string;
  "windowControlsLinux.hoverBackground": string;
  "windowControlsLinux.activeBackground": string;
  dvBackgroundColor?: string;
  dvPaneviewActiveOutlineColor?: string;
  dvTabsAndActionsContainerFontSize?: string;
  dvTabsAndActionsContainerHeight?: string;
  dvDragOverBackgroundColor?: string;
  dvDragOverBorderColor?: string;
  dvTabsContainerScrollbarColor?: string;
  dvIconHoverBackgroundColor?: string;
  dvFloatingBoxShadow?: string;
  dvGroupViewBackgroundColor?: string;
  dvTabsAndActionsContainerBackgroundColor?: string;
  dvActivegroupVisiblepanelTabBackgroundColor?: string;
  dvActivegroupHiddenpanelTabBackgroundColor?: string;
  dvInactivegroupVisiblepanelTabBackgroundColor?: string;
  dvInactivegroupHiddenpanelTabBackgroundColor?: string;
  dvTabDividerColor?: string;
  dvActivegroupVisiblepanelTabColor?: string;
  dvActivegroupHiddenpanelTabColor?: string;
  dvInactivegroupVisiblepanelTabColor?: string;
  dvInactivegroupHiddenpanelTabColor?: string;
  dvSeparatorBorder?: string;
  dvPaneviewHeaderBorderColor?: string;
}

const themes: InputTheme[] = [
  {
    name: "moss-light",
    type: "light",
    default: true,
    colors: {
      primary: "0, 0, 0, 1",
      "sideBar.background": "244, 244, 245, 1",
      "toolBar.background": "224, 224, 224, 1",
      "page.background": "255,255,255, 1",
      "statusBar.background": "0, 122, 205, 1",
      "windowsCloseButton.background": "196, 43, 28, 1",
      "windowControlsLinux.background": "218, 218, 218, 1",
      "windowControlsLinux.text": "61, 61, 61, 1",
      "windowControlsLinux.hoverBackground": "209, 209, 209, 1",
      "windowControlsLinux.activeBackground": "191, 191, 191, 1",
      dvBackgroundColor: "0, 0, 0, 1", // Black
      dvPaneviewActiveOutlineColor: "30, 144, 255, 1", // DodgerBlue
      dvTabsAndActionsContainerFontSize: "13px",
      dvTabsAndActionsContainerHeight: "35px",
      dvDragOverBackgroundColor: "83, 89, 93, 0.5",
      dvDragOverBorderColor: "255, 255, 255, 1", // White
      dvTabsContainerScrollbarColor: "136, 136, 136, 1", // #888
      dvIconHoverBackgroundColor: "90, 93, 94, 0.31",
      dvFloatingBoxShadow: "8px 8px 8px 0px rgba(83, 89, 93, 0.5)",
      dvGroupViewBackgroundColor: "255, 255, 255, 1", // White
      dvTabsAndActionsContainerBackgroundColor: "243, 243, 243, 1", // #f3f3f3
      dvActivegroupVisiblepanelTabBackgroundColor: "255, 255, 255, 1", // White
      dvActivegroupHiddenpanelTabBackgroundColor: "236, 236, 236, 1", // #ececec
      dvInactivegroupVisiblepanelTabBackgroundColor: "255, 255, 255, 1", // White
      dvInactivegroupHiddenpanelTabBackgroundColor: "236, 236, 236, 1", // #ececec
      dvTabDividerColor: "255, 255, 255, 1", // White
      dvActivegroupVisiblepanelTabColor: "51, 51, 51, 1", // rgb(51, 51, 51)
      dvActivegroupHiddenpanelTabColor: "51, 51, 51, 0.7", // rgba(51, 51, 51, 0.7)
      dvInactivegroupVisiblepanelTabColor: "51, 51, 51, 0.7", // rgba(51, 51, 51, 0.7)
      dvInactivegroupHiddenpanelTabColor: "51, 51, 51, 0.35", // rgba(51, 51, 51, 0.35)
      dvSeparatorBorder: "128, 128, 128, 0.35", // rgba(128, 128, 128, 0.35)
      dvPaneviewHeaderBorderColor: "51, 51, 51, 1", // rgb(51, 51, 51)
    },
  },
  {
    name: "moss-dark",
    type: "dark",
    default: false,
    colors: {
      primary: "255, 255, 255, 1",
      "sideBar.background": "39, 39, 42, 1",
      "toolBar.background": "30, 32, 33, 1",
      "page.background": "22, 24, 25, 1",
      "statusBar.background": "0, 122, 205, 1",
      "windowsCloseButton.background": "196, 43, 28, 1",
      "windowControlsLinux.background": "55, 55, 55, 1",
      "windowControlsLinux.text": "255, 255, 255, 1",
      "windowControlsLinux.hoverBackground": "66, 66, 66, 1",
      "windowControlsLinux.activeBackground": "86, 86, 86, 1",
      dvBackgroundColor: "0, 0, 0, 1",
      dvPaneviewActiveOutlineColor: "30, 144, 255, 1", // DodgerBlue
      dvTabsAndActionsContainerFontSize: "13px",
      dvTabsAndActionsContainerHeight: "35px",
      dvDragOverBackgroundColor: "83, 89, 93, 0.5",
      dvDragOverBorderColor: "255, 255, 255, 1", // White
      dvTabsContainerScrollbarColor: "136, 136, 136, 1", // #888
      dvIconHoverBackgroundColor: "90, 93, 94, 0.31",
      dvFloatingBoxShadow: "8px 8px 8px 0px rgba(83, 89, 93, 0.5)",
      dvGroupViewBackgroundColor: "30, 30, 30, 1", // #1e1e1e
      dvTabsAndActionsContainerBackgroundColor: "37, 37, 38, 1", // #252526
      dvActivegroupVisiblepanelTabBackgroundColor: "30, 30, 30, 1", // #1e1e1e
      dvActivegroupHiddenpanelTabBackgroundColor: "45, 45, 45, 1", // #2d2d2d
      dvInactivegroupVisiblepanelTabBackgroundColor: "30, 30, 30, 1", // #1e1e1e
      dvInactivegroupHiddenpanelTabBackgroundColor: "45, 45, 45, 1", // #2d2d2d
      dvTabDividerColor: "30, 30, 30, 1", // #1e1e1e
      dvActivegroupVisiblepanelTabColor: "255, 255, 255, 1", // White
      dvActivegroupHiddenpanelTabColor: "150, 150, 150, 1", // #969696
      dvInactivegroupVisiblepanelTabColor: "143, 143, 143, 1", // #8f8f8f
      dvInactivegroupHiddenpanelTabColor: "98, 98, 98, 1", // #626262
      dvSeparatorBorder: "68, 68, 68, 1", // rgb(68, 68, 68)
      dvPaneviewHeaderBorderColor: "204, 204, 204, 0.2",
    },
  },
  {
    name: "moss-pink",
    type: "pink",
    default: false,
    colors: {
      primary: "0, 0, 0, 1",
      "sideBar.background": "234, 157, 242, 1",
      "toolBar.background": "222, 125, 232, 1",
      "page.background": "227, 54, 245, 1",
      "statusBar.background": "63, 11, 69, 1",
      "windowsCloseButton.background": "196, 43, 28, 1",
      "windowControlsLinux.background": "218, 218, 218, 1",
      "windowControlsLinux.text": "61, 61, 61, 1",
      "windowControlsLinux.hoverBackground": "209, 209, 209, 1",
      "windowControlsLinux.activeBackground": "191, 191, 191, 1",

      dvTabsAndActionsContainerBackgroundColor: "255, 18, 255, 1",
      dvActivegroupVisiblepanelTabBackgroundColor: "255, 0, 255, 1", // Magenta
      dvActivegroupHiddenpanelTabBackgroundColor: "128, 0, 128, 1", // Purple
      dvActivegroupVisiblepanelTabColor: "255, 255, 255, 1", // White
      dvActivegroupHiddenpanelTabColor: "255, 255, 255, 1", // White
    },
  },
];

function ensureDirectoryExists(directory: string): void {
  if (!existsSync(directory)) {
    mkdirSync(directory);
  }
}

async function writeThemeFile(theme: InputTheme): Promise<void> {
  const fileName = `${themesDirectory}/${theme.name}.json`;
  writeFileSync(fileName, JSON.stringify(theme, null, 2), {
    flag: "w",
  });
}

async function generateThemeFiles(): Promise<void> {
  try {
    await ensureDirectoryExists(themesDirectory);
    await Promise.all(themes.map(writeThemeFile));
    console.log("Theme files generated successfully.");
  } catch (error) {
    console.error("Error generating theme files:", error);
    process.exit(1);
  }
}

generateThemeFiles();
