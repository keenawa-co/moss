import { useContext, type HTMLProps } from "react";

import { cn } from "@repo/moss-ui";

import { ControlButton } from "./ControlButton";
import ControlsContext from "./ControlsContext";
import { ControlsIcons } from "./icons";

// FIXME: fix opacity bg-[rgba(var(--moss-windows-close-button-background))]/90
// FIXME: analyze and fix dark:...

export function WindowsControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(ControlsContext);

  return (
    <div className={cn("flex h-full", className)} {...props}>
      <ControlButton
        onClick={minimizeWindow}
        className="text-[--moss-primary]/90 active:background-[--moss-primary]/[.03] h-full w-[46px] cursor-default rounded-none bg-transparent hover:bg-[#0000000d]"
      >
        <ControlsIcons.minimizeWin />
      </ControlButton>
      <ControlButton
        onClick={maximizeWindow}
        className={cn(
          "h-full w-[46px] cursor-default rounded-none bg-transparent",
          "text-[--moss-primary]/90 active:background-[--moss-primary]/[.03] hover:bg-[#0000000d]"
        )}
      >
        {isWindowMaximized ? <ControlsIcons.maximizeRestoreWin /> : <ControlsIcons.maximizeWin />}
      </ControlButton>
      <ControlButton
        onClick={closeWindow}
        className="text-[--moss-primary]/90 active:background-[--moss-windowsCloseButton-background]/90 h-full w-[46px] cursor-default rounded-none bg-transparent hover:text-white hover:background-[--moss-windowsCloseButton-background]"
      >
        <ControlsIcons.closeWin />
      </ControlButton>
    </div>
  );
}
