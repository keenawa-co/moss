import type { Config } from "tailwindcss";

import sharedConfig from "@repo/tailwind-config";
import tailwindTypography from "@tailwindcss/typography";

const config: Pick<Config, "content" | "presets" | "theme" | "plugins"> = {
  content: ["./src/**/*.{js,ts,jsx,tsx}", "../../packages/moss-ui/src/**/*.{js,ts,jsx,tsx,mdx}"],
  presets: [sharedConfig],
  theme: {
    extend: {
      colors: {
        "dv-paneview-header-border-color": "var(--dv-paneview-header-border-color)",
        "dv-group-view-background-color": "var(--dv-group-view-background-color)",
        "dv-activegroup-visiblepanel-tab-color": "var(--dv-activegroup-visiblepanel-tab-color)",
        "dv-paneview-active-outline-color": "var(--dv-paneview-active-outline-color)",
      },
      backgroundColor: {
        "dv-enabled": "black",
        "dv-disabled": "orange",
        "dv-maximum": "green",
        "dv-minimum": "red",
        "dv-group-view": "var(--dv-group-view-background-color)",
      },
      borderColor: {
        "dv-paneview-header": "var(--dv-paneview-header-border-color)",
      },
      outlineColor: {
        "dv-paneview-active": "var(--dv-paneview-active-outline-color)",
      },
      cursor: {
        "ew-resize": "ew-resize",
        "ns-resize": "ns-resize",
        "w-resize": "w-resize",
        "e-resize": "e-resize",
        "n-resize": "n-resize",
        "s-resize": "s-resize",
        "pointer": "pointer",
      },
      transitionDuration: {
        "150": "0.15s",
      },
      transitionTimingFunction: {
        "ease-out": "ease-out",
      },
      zIndex: {
        "5": "5",
        "99": "99",
      },
      inset: {
        "0": "0",
      },
      width: {
        "1": "1px",
        "4": "4px",
        "100": "100%",
        "full": "100%",
      },
      height: {
        "1": "1px",
        "4": "4px",
        "100": "100%",
        "full": "100%",
      },
      padding: {
        "0": "0px",
        "2": "0.5rem",
        "8": "8px",
      },
      flexGrow: {
        "1": "1",
      },
      flexDirection: {
        "col": "column",
      },
      overflow: {
        "hidden": "hidden",
        "auto": "auto",
      },
      position: {
        "relative": "relative",
      },
      outline: {
        "none": "none",
      },
      display: {
        "flex": "flex",
      },
      justifyContent: {
        "center": "center",
      },
      alignItems: {
        "center": "center",
      },
      boxSizing: {
        "border-box": "border-box",
      },
      userSelect: {
        "none": "none",
      },
      paddingLeft: {
        "2": "0.5rem",
      },
      borderTopWidth: {
        "0": "0",
      },
    },
  },
  plugins: [tailwindTypography],
};

export default config;
