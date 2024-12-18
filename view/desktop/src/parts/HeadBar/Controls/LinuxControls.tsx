import { useContext, type HTMLProps } from "react";

import { cn } from "@repo/moss-ui";

import { ControlButton } from "./ControlButton";
import ControlsContext from "./ControlsContext";
import { ControlsIcons } from "./icons";

export function LinuxControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(ControlsContext);

  return (
    <div className={cn("mr-2.5 flex h-auto items-center space-x-[13px]", className)} {...props}>
      <ControlButton
        onClick={minimizeWindow}
        className="size-6 cursor-default rounded-full text-[--color-windowControlsLinux-text] background-[--color-windowControlsLinux-background] hover:background-[--color-windowControlsLinux-hoverBackground] active:background-[--color-windowControlsLinux-activeBackground] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <ControlsIcons.minimizeWin className="size-[9px]" />
      </ControlButton>
      <ControlButton
        onClick={maximizeWindow}
        className="size-6 cursor-default rounded-full text-[--color-windowControlsLinux-text] background-[--color-windowControlsLinux-background] hover:background-[--color-windowControlsLinux-hoverBackground] active:background-[--color-windowControlsLinux-activeBackground] dark:text-white dark:background-[#373737] dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        {isWindowMaximized ? (
          <ControlsIcons.maximizeRestoreWin className="size-[9px]" />
        ) : (
          <ControlsIcons.maximizeWin className="size-2" />
        )}
      </ControlButton>
      <ControlButton
        onClick={closeWindow}
        className="size-6 cursor-default rounded-full text-[--color-windowControlsLinux-text] background-[--color-windowControlsLinux-background] hover:background-[--color-windowControlsLinux-hoverBackground] active:background-[--color-windowControlsLinux-activeBackground] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <ControlsIcons.closeWin className="size-2" />
      </ControlButton>
    </div>
  );
}
