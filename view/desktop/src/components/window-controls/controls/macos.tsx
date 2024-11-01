import { useContext, useEffect, useState, type HTMLProps } from "react";
import { Icons } from "@/components/window-controls/components/icons";
import { cn } from "@/components/window-controls/libs/utils";
import { Button } from "@/components/window-controls/components/button";
import { TauriAppWindowContext } from "@/components/window-controls/contexts";

export function MacOS({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { minimizeWindow, maximizeWindow, fullscreenWindow, closeWindow } = useContext(TauriAppWindowContext);

  const [isAltKeyPressed, setIsAltKeyPressed] = useState(false);
  const [isHovering, setIsHovering] = useState(false);

  const last = isAltKeyPressed ? <Icons.plusMac /> : <Icons.fullMac />;
  const key = "Alt";

  const handleMouseEnter = () => {
    setIsHovering(true);
  };
  const handleMouseLeave = () => {
    setIsHovering(false);
  };

  const handleAltKeyDown = (e: KeyboardEvent) => {
    if (e.key === key) {
      setIsAltKeyPressed(true);
    }
  };
  const handleAltKeyUp = (e: KeyboardEvent) => {
    if (e.key === key) {
      setIsAltKeyPressed(false);
    }
  };
  useEffect(() => {
    window.addEventListener("keydown", handleAltKeyDown);
    window.addEventListener("keyup", handleAltKeyUp);
  }, []);

  return (
    <div
      className={cn("text-black active:text-black dark:text-black space-x-2 px-3", className)}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      {...props}
    >
      <Button
        onClick={closeWindow}
        className="border-black/[.12] text-black/60 active:text-black/60 aspect-square h-3 w-3 cursor-default content-center items-center justify-center self-center rounded-full border bg-[#ff544d] text-center hover:bg-[#ff544d] active:bg-[#bf403a] dark:border-none"
      >
        {isHovering && <Icons.closeMac />}
      </Button>
      <Button
        onClick={minimizeWindow}
        className="border-black/[.12] text-black/60 active:text-black/60 aspect-square h-3 w-3 cursor-default content-center items-center justify-center self-center  rounded-full border bg-[#ffbd2e] text-center hover:bg-[#ffbd2e] active:bg-[#bf9122] dark:border-none"
      >
        {isHovering && <Icons.minMac />}
      </Button>
      <Button
        // onKeyDown={handleAltKeyDown}
        // onKeyUp={handleAltKeyUp}
        onClick={isAltKeyPressed ? maximizeWindow : fullscreenWindow}
        className="border-black/[.12] text-black/60 active:text-black/60 aspect-square h-3 w-3 cursor-default content-center items-center justify-center self-center rounded-full border bg-[#28c93f] text-center hover:bg-[#28c93f] active:bg-[#1e9930] dark:border-none"
      >
        {isHovering && last}
      </Button>
    </div>
  );
}
