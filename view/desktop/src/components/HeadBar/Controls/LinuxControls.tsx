import { useState, type HTMLProps } from "react";
import { ControlButton } from "./ControlButton";
import { Icons } from "./icons";
import { cn } from "@repo/ui";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function LinuxControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const apiWindow = getCurrentWindow();
  const [isMaximized, setIsMaximized] = useState(false);

  const handleToggleMaximize = async () => {
    const isMaximized = await apiWindow.isMaximized();
    setIsMaximized(isMaximized);
    await apiWindow.toggleMaximize();
  };

  return (
    <div className={cn("mr-[10px] flex h-auto items-center space-x-[13px]", className)} {...props}>
      <ControlButton
        onClick={() => apiWindow.minimize()}
        className="m-0 h-6 w-6 cursor-default rounded-full bg-[rgba(var(--color-window-controls-linux-background))] p-0 text-[rgba(var(--color-window-controls-linux-text))] hover:bg-[rgba(var(--color-window-controls-linux-hover-background))] active:bg-[rgba(var(--color-window-controls-linux-active-background))] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <Icons.minimizeWin className="h-[9px] w-[9px]" />
      </ControlButton>
      <ControlButton
        onClick={handleToggleMaximize}
        className="m-0 h-6 w-6 cursor-default rounded-full bg-[rgba(var(--color-window-controls-linux-background))] p-0 text-[rgba(var(--color-window-controls-linux-text))] hover:bg-[rgba(var(--color-window-controls-linux-hover-background))] active:bg-[rgba(var(--color-window-controls-linux-active-background))] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        {isMaximized ? (
          <Icons.maximizeWin className="h-2 w-2" />
        ) : (
          <Icons.maximizeRestoreWin className="h-[9px] w-[9px]" />
        )}
      </ControlButton>
      <ControlButton
        onClick={() => apiWindow.close()}
        className="m-0 h-6 w-6 cursor-default rounded-full bg-[rgba(var(--color-window-controls-linux-background))] p-0 text-[rgba(var(--color-window-controls-linux-text))] hover:bg-[rgba(var(--color-window-controls-linux-hover-background))] active:bg-[rgba(var(--color-window-controls-linux-active-background))] dark:bg-[#373737] dark:text-white dark:hover:bg-[#424242] dark:active:bg-[#565656]"
      >
        <Icons.closeWin className="h-2 w-2" />
      </ControlButton>
    </div>
  );
}
