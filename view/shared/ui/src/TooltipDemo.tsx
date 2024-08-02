import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "./shadcn/Tooltip";
import { useState } from "react";

// import keymap from "native-keymap";
// var keymap = require("native-keymap");

const TooltipDemo = ({
  label,
  shortcut,
  onOpenChange,
  disabled,
  children,
  open = false,
}: {
  open?: boolean;
  label?: string;
  shortcut?: string[];
  onOpenChange?: (e: Event) => void;
  disabled?: boolean;
  children?: React.ReactNode;
}) => {
  // console.log(keymap);
  return (
    <TooltipProvider>
      <Tooltip delayDuration={0} open={open} onOpenChange={(e) => e.preventDefault() && onOpenChange}>
        <TooltipTrigger asChild onContextMenu={(e) => disabled && e.preventDefault()}>
          {children}
        </TooltipTrigger>
        <TooltipContent>
          <div>
            {label} {shortcut && <span className=" text-neutral-400 uppercase">{shortcut.map((s) => s)}</span>}
          </div>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};

export default TooltipDemo;
