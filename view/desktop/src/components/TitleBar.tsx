import { twMerge } from "tailwind-merge";
import { WindowTitlebar } from "@/components";

interface TitleBarProps {
  showControls?: string;
  isMacOs?: boolean;
}

export const TitleBar = ({}: TitleBarProps) => {
  return (
    <header data-tauri-drag-region className={twMerge("absolute inset-0 h-11 bg-toolBarBackground")}>
      <WindowTitlebar />
    </header>
  );
};
