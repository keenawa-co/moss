import { useContext, useEffect, useState, type HTMLProps } from "react";
import { Icons } from "@/components/window-controls/components/icons";
import { cn } from "@/components/window-controls/libs/utils";
import { Button } from "@/components/window-controls/components/button";
import { TauriAppWindowContext } from "@/components/window-controls/contexts";

const buttonStyles = `text-black/60 active:text-black/60 size-[12px] cursor-default grid  items-center justify-center self-center rounded-full `;
const iconStyles = `hidden size-1.5 group-hover:block`;

export function MacOS({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { minimizeWindow, maximizeWindow, fullscreenWindow, closeWindow } = useContext(TauriAppWindowContext);

  const [isAltKeyPressed, setIsAltKeyPressed] = useState(false);

  useEffect(() => {
    window.addEventListener("keydown", (e: KeyboardEvent) => {
      if (e.key === "Alt") setIsAltKeyPressed(true);
    });
    window.addEventListener("keyup", (e: KeyboardEvent) => {
      if (e.key === "Alt") setIsAltKeyPressed(false);
    });
  }, []);

  return (
    <div className={cn("text-black group flex gap-2 px-4", className)} {...props}>
      <Button
        onClick={closeWindow}
        className={cn(buttonStyles, "bg-[#ff544d] hover:bg-[#ff544d] active:bg-[#bf403a] dark:border-none")}
      >
        <Icons.closeMac className={cn(iconStyles)} />
      </Button>
      <Button
        onClick={minimizeWindow}
        className={cn(buttonStyles, "bg-[#ffbd2e] hover:bg-[#ffbd2e] active:bg-[#bf9122] dark:border-none")}
      >
        <Icons.minMac className={cn(iconStyles)} />
      </Button>
      <Button
        onClick={isAltKeyPressed ? maximizeWindow : fullscreenWindow}
        className={cn(buttonStyles, "bg-[#28c93f] hover:bg-[#28c93f] active:bg-[#1e9930] dark:border-none")}
      >
        {isAltKeyPressed ? <Icons.plusMac className={cn(iconStyles)} /> : <Icons.fullMac className={cn(iconStyles)} />}
      </Button>
    </div>
  );
}
