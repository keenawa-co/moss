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
        className="active:background-(--moss-primary)/[.03] h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-primary)/90 hover:bg-[#0000000d]"
      >
        <ControlsIcons.minimizeWin />
      </ControlButton>
      <ControlButton
        onClick={maximizeWindow}
        className={cn(
          "h-full w-[46px] cursor-default rounded-none bg-transparent",
          "active:background-(--moss-primary)/[.03] text-(--moss-primary)/90 hover:bg-[#0000000d]"
        )}
      >
        {isWindowMaximized ? <ControlsIcons.maximizeRestoreWin /> : <ControlsIcons.maximizeWin />}
      </ControlButton>
      <ControlButton
        onClick={closeWindow}
        className="active:background-(--moss-windowsCloseButton-background)/90 hover:background-(--moss-windowsCloseButton-background) h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-primary)/90 hover:text-white"
      >
        <ControlsIcons.closeWin />
      </ControlButton>
    </div>
  );
}
