import { useContext, type HTMLProps } from "react";
import { Icons } from "@/components/window-controls/components/icons";
import TauriAppWindowContext from "@/components/window-controls/contexts/plugin-window";
import { cn } from "@/components/window-controls/libs/utils";
import { Button } from "@/components/window-controls/components/button";

export function Windows({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(TauriAppWindowContext);

  return (
    <div className={cn("h-8", className)} {...props}>
      <Button
        onClick={minimizeWindow}
        className="max-h-8 w-[46px] cursor-default rounded-none bg-transparent text-primary/90 hover:bg-primary/[.05] active:bg-primary/[.03]  dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
      >
        <Icons.minimizeWin />
      </Button>
      <Button
        onClick={maximizeWindow}
        className={cn(
          "max-h-8 w-[46px] cursor-default rounded-none bg-transparent",
          "text-primary/90 hover:bg-primary/[.05] active:bg-primary/[.03] dark:text-white dark:hover:bg-white/[.06] dark:active:bg-white/[.04]"
          // !isMaximizable && "text-white/[.36]",
        )}
      >
        {!isWindowMaximized ? <Icons.maximizeWin /> : <Icons.maximizeRestoreWin />}
      </Button>
      <Button
        onClick={closeWindow}
        className="max-h-8 w-[46px] cursor-default rounded-none bg-transparent text-primary/90 hover:bg-windows-close-button-background hover:text-white active:bg-windows-close-button-background/90 dark:text-white"
      >
        <Icons.closeWin />
      </Button>
    </div>
  );
}
