import { useState, type HTMLProps } from "react";
import { ControlButton } from "./ControlButton";
import { Icons } from "./icons";
import { cn } from "@repo/ui";
import { getCurrentWindow } from "@tauri-apps/api/window";

// FIXME: fix opacity bg-[rgba(var(--color-windows-close-button-background))]/90
// FIXME: analyze and fix dark:...

export function WindowsControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const apiWindow = getCurrentWindow();
  const [isMaximized, setIsMaximized] = useState(false);

  const handleToggleMaximize = async () => {
    const isMaximized = await apiWindow.isMaximized();
    setIsMaximized(isMaximized);
    await apiWindow.toggleMaximize();
  };

  return (
    <div className={cn("flex h-full", className)} {...props}>
      <ControlButton
        onClick={() => apiWindow.minimize()}
        className="h-full w-[46px] cursor-default rounded-none bg-transparent text-[rgba(var(--color-primary))]/90 hover:bg-[#0000000d] active:bg-[rgba(var(--color-primary))]/[.03]  dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
      >
        <Icons.minimizeWin />
      </ControlButton>
      <ControlButton
        onClick={handleToggleMaximize}
        className={cn(
          "h-full w-[46px] cursor-default rounded-none bg-transparent",
          "text-[rgba(var(--color-primary))]/90 hover:bg-[#0000000d] active:bg-[rgba(var(--color-primary))]/[.03] dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
        )}
      >
        {isMaximized ? <Icons.maximizeWin /> : <Icons.maximizeRestoreWin />}
      </ControlButton>
      <ControlButton
        onClick={() => apiWindow.close()}
        className="h-full w-[46px] cursor-default rounded-none bg-transparent text-[rgba(var(--color-primary))]/90 hover:bg-[rgba(var(--color-windows-close-button-background))] hover:text-white active:bg-[rgba(var(--color-windows-close-button-background))]/90 dark:text-white"
      >
        <Icons.closeWin />
      </ControlButton>
    </div>
  );
}
