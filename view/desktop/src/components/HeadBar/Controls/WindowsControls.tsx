import { useContext, type HTMLProps } from "react";
import { ControlButton } from "./ControlButton";
import TauriAppWindowContext from "./plugin-window";
import { Icons } from "./icons";
import { cn } from "@repo/ui";

// FIXME: fix opacity bg-[rgba(var(--color-windows-close-button-background))]/90
// FIXME: analyze and fix dark:...

export function WindowsControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(TauriAppWindowContext);

  return (
    <div className={cn("flex h-full", className)} {...props}>
      <ControlButton
        onClick={minimizeWindow}
        className="h-full w-[46px] cursor-default rounded-none bg-transparent text-[rgba(var(--color-primary))]/90 hover:bg-[#0000000d] active:bg-[rgba(var(--color-primary))]/[.03]  dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
      >
        <Icons.minimizeWin />
      </ControlButton>
      <ControlButton
        onClick={maximizeWindow}
        className={cn(
          "h-full w-[46px] cursor-default rounded-none bg-transparent",
          "text-[rgba(var(--color-primary))]/90 hover:bg-[#0000000d] active:bg-[rgba(var(--color-primary))]/[.03] dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
        )}
      >
        {!isWindowMaximized ? <Icons.maximizeWin /> : <Icons.maximizeRestoreWin />}
      </ControlButton>
      <ControlButton
        onClick={closeWindow}
        className="h-full w-[46px] cursor-default rounded-none bg-transparent text-[rgba(var(--color-primary))]/90 hover:bg-[rgba(var(--color-windows-close-button-background))] hover:text-white active:bg-[rgba(var(--color-windows-close-button-background))]/90 dark:text-white"
      >
        <Icons.closeWin />
      </ControlButton>
    </div>
  );
}
