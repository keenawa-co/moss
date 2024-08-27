import React, { useContext, type HTMLProps } from "react";
import { Button } from "@/components/window-controls/components/button";
import { Icons } from "@/components/window-controls/components/icons";
import TauriAppWindowContext from "@/components/window-controls/contexts/plugin-window";
import { cn } from "@/components/window-controls/libs/utils";

export function Gnome({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(TauriAppWindowContext);

  return (
    <div className={cn("mr-[10px] h-auto items-center space-x-[13px]", className)} {...props}>
      <Button
        onClick={minimizeWindow}
        className="m-0 aspect-square h-6 w-6 cursor-default rounded-full bg-window-controls-linux-background p-0 text-window-controls-linux-text hover:bg-window-controls-linux-background-hover active:bg-window-controls-linux-background-active dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <Icons.minimizeWin className="h-[9px] w-[9px]" />
      </Button>
      <Button
        onClick={maximizeWindow}
        className="m-0 aspect-square h-6 w-6 cursor-default rounded-full bg-window-controls-linux-background p-0 text-window-controls-linux-text hover:bg-window-controls-linux-background-hover active:bg-window-controls-linux-background-active dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        {!isWindowMaximized ? (
          <Icons.maximizeWin className="h-2 w-2" />
        ) : (
          <Icons.maximizeRestoreWin className="h-[9px] w-[9px]" />
        )}
      </Button>
      <Button
        onClick={closeWindow}
        className="m-0 aspect-square h-6 w-6 cursor-default rounded-full bg-window-controls-linux-background p-0 text-window-controls-linux-text hover:bg-window-controls-linux-background-hover active:bg-window-controls-linux-background-active dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <Icons.closeWin className="h-2 w-2" />
      </Button>
    </div>
  );
}
