import { useContext, type HTMLProps } from "react";
import { Icons } from "@/components/window-controls/components/icons";
import TauriAppWindowContext from "@/components/window-controls/contexts/plugin-window";
import { cn } from "@/components/window-controls/libs/utils";
import { Button } from "@/components/window-controls/components/button";

// FIXME: fix opacity bg-[rgba(var(--color-windows-close-button-background))]/90
// FIXME: analyze and fix dark:...

export function Windows({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(TauriAppWindowContext);

  return (
    <div className={cn("h-full", className)} {...props}>
      <Button
        onClick={minimizeWindow}
        className="h-full w-[46px] cursor-default rounded-none bg-transparent text-[rgba(var(--color-primary))]/90 hover:bg-[rgba(var(--color-primary))]/[.05] active:bg-[rgba(var(--color-primary))]/[.03]  dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
      >
        <Icons.minimizeWin />
      </Button>
      <Button
        onClick={maximizeWindow}
        className={cn(
          "h-full w-[46px] cursor-default rounded-none bg-transparent",
          "text-[rgba(var(--color-primary))]/90 hover:bg-[rgba(var(--color-primary))]/[.05] active:bg-[rgba(var(--color-primary))]/[.03] dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
          // !isMaximizable && "text-white/[.36]",
        )}
      >
        {!isWindowMaximized ? <Icons.maximizeWin /> : <Icons.maximizeRestoreWin />}
      </Button>
      <Button
        onClick={closeWindow}
        className="h-full w-[46px] cursor-default rounded-none bg-transparent text-[rgba(var(--color-primary))]/90 hover:bg-[rgba(var(--color-windows-close-button-background))] hover:text-white active:bg-[rgba(var(--color-windows-close-button-background))]/90 dark:text-white"
      >
        <Icons.closeWin />
      </Button>
    </div>
  );
}
