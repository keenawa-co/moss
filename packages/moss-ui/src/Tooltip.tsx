import React from "react";

import * as TooltipPrimitive from "@radix-ui/react-tooltip";

import { Link } from "./Link";
import { cn } from "./utils";

// api-reference https://www.radix-ui.com/primitives/docs/components/tooltip#api-reference

export interface TooltipOptions {
  provider?: Pick<
    TooltipPrimitive.TooltipProviderProps,
    "delayDuration" | "skipDelayDuration" | "disableHoverableContent"
  >;
  root?: Pick<
    TooltipPrimitive.TooltipProps,
    "defaultOpen" | "open" | "onOpenChange" | "delayDuration" | "disableHoverableContent"
  >;
  content?: Pick<
    TooltipPrimitive.TooltipContentProps,
    | "aria-label"
    | "onEscapeKeyDown"
    | "onPointerDownOutside"
    | "forceMount"
    | "side"
    | "sideOffset"
    | "align"
    | "alignOffset"
    | "avoidCollisions"
    | "collisionBoundary"
    | "collisionPadding"
    | "arrowPadding"
    | "sticky"
    | "hideWhenDetached"
  >;
  arrow?: Pick<TooltipPrimitive.TooltipArrowProps, "asChild" | "width" | "height">;
  portal?: Pick<TooltipPrimitive.TooltipPortalProps, "forceMount" | "container">;
}

const TooltipShortcut = ({ shortcut }: { shortcut: string[] }) => {
  return (
    <div className="font-medium text-[#818594] uppercase">
      {shortcut.map((s) => (
        <span key={s}>{s}</span>
      ))}
    </div>
  );
};

export const Tooltip = ({
  header,
  text,
  shortcut, //TODO shortcut doesn't have any functionality
  link,
  options,
  arrow = false,
  asChild = false,
  children,
  className,
}: {
  header?: string;
  text?: string;
  label?: string;
  link?: {
    label: string;
    url: string;
  };
  shortcut?: string[];
  options?: TooltipOptions;
  arrow?: boolean;
  asChild?: boolean;
  className?: string;
  children?: React.ReactNode;
}) => {
  return (
    <TooltipPrimitive.Provider {...options?.provider}>
      <TooltipPrimitive.Root {...options?.root}>
        <TooltipPrimitive.Trigger asChild={asChild}>{children}</TooltipPrimitive.Trigger>
        <TooltipPrimitive.Content
          className={cn(
            "z-50 -mb-px flex max-w-80 overflow-hidden rounded-md bg-[#1E1E1E] leading-5 text-white shadow-md",
            className
          )}
          {...options?.content}
        >
          {options?.portal && <TooltipPrimitive.Portal {...options?.portal} />}
          {arrow && <TooltipPrimitive.Arrow {...options?.arrow} className="fill-[#1E1E1E]" />}

          {text || link ? (
            <div className="flex flex-col items-start gap-1.5 px-4 py-3">
              {header && <div className="font-medium">{header}</div>}

              {text && <div className="text-[#C9CCD6]">{text}</div>}

              {shortcut && <TooltipShortcut shortcut={shortcut} />}

              {link && <Link {...link} type="secondary" withIcon />}
            </div>
          ) : (
            <div className="flex gap-1.5 p-2">
              {header && <div className="font-medium">{header}</div>}

              {shortcut && <TooltipShortcut shortcut={shortcut} />}
            </div>
          )}
        </TooltipPrimitive.Content>
      </TooltipPrimitive.Root>
    </TooltipPrimitive.Provider>
  );
};

export default Tooltip;
