import { twMerge } from "tailwind-merge";
import { WindowTitlebar } from "@/components";

interface TitleBarProps {
  showControls?: string;
  isMacOs?: boolean;
}

export const TitleBar = ({}: TitleBarProps) => {
  return (
    <header data-tauri-drag-region className={twMerge("inset-0 bg-toolbar-background")}>
      <WindowTitlebar />
    </header>
  );
};
