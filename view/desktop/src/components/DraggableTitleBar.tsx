import { twMerge } from "tailwind-merge";
import { MacOsTrafficLights } from "@/components";

interface DraggableTitleBarProps {
  showControls?: string;
  isMacOs?: boolean;
}

//FIXME: Remove pixels
export const DraggableTitleBar = ({}: DraggableTitleBarProps) => {
  return (
    <header data-tauri-drag-region className={twMerge("absolute inset-0 h-11 bg-zinc-100")}>
      <MacOsTrafficLights className="absolute left-[13px] top-[13px] z-50" />
    </header>
  );
};
