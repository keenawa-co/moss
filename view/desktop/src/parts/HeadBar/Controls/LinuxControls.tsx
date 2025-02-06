import { useContext, type HTMLProps } from "react";

import { cn } from "@/utils";

import { ControlButton } from "./ControlButton";
import ControlsContext from "./ControlsContext";
import { ControlsIcons } from "./icons";

export function LinuxControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(ControlsContext);

  return (
    <div className={cn("mr-2.5 flex h-auto items-center space-x-[13px]", className)} {...props}>
      <ControlButton
        onClick={minimizeWindow}
        className="background-(--moss-windowControlsLinux-background) hover:background-(--moss-windowControlsLinux-hoverBackground) active:background-(--moss-windowControlsLinux-activeBackground) size-6 cursor-default rounded-full text-(--moss-windowControlsLinux-text) dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <ControlsIcons.minimizeWin className="size-[9px]" />
      </ControlButton>
      <ControlButton
        onClick={maximizeWindow}
        className="background-(--moss-windowControlsLinux-background) hover:background-(--moss-windowControlsLinux-hoverBackground) active:background-(--moss-windowControlsLinux-activeBackground) dark:background-[#373737] size-6 cursor-default rounded-full text-(--moss-windowControlsLinux-text) dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        {isWindowMaximized ? (
          <ControlsIcons.maximizeRestoreWin className="size-[9px]" />
        ) : (
          <ControlsIcons.maximizeWin className="size-2" />
        )}
      </ControlButton>
      <ControlButton
        onClick={closeWindow}
        className="background-(--moss-windowControlsLinux-background) hover:background-(--moss-windowControlsLinux-hoverBackground) active:background-(--moss-windowControlsLinux-activeBackground) size-6 cursor-default rounded-full text-(--moss-windowControlsLinux-text) dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <ControlsIcons.closeWin className="size-2" />
      </ControlButton>
    </div>
  );
}
