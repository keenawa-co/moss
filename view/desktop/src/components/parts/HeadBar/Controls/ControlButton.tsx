import { cn } from "../../../../../../packages/moss-ui/src";
import type { ButtonHTMLAttributes } from "react";

export function ControlButton({ className, children, ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button className={cn("inline-flex cursor-default items-center justify-center", className)} {...props}>
      {children}
    </button>
  );
}
