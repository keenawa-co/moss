import { ComponentPropsWithoutRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as icons from "@repo/icongen";

export type Icons = keyof typeof icons;

interface IconProps extends ComponentPropsWithoutRef<"svg"> {
  icon: Icons | null;
  className?: string;
}

export const Icon = forwardRef<SVGSVGElement, IconProps>(({ icon, className, ...props }, forwardedRef) => {
  if (icon && icons[icon]) {
    const IconTag = icons[icon];
    return <IconTag ref={forwardedRef} className={cn("size-4 shrink-0 text-current", className)} {...props} />;
  }

  return <FailedIcon />;
});

export default Icon;

const FailedIcon = ({ className, ...props }: { className?: string } & ComponentPropsWithoutRef<"svg">) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 16 16"
      className={cn("size-4 shrink-0 bg-amber-500", className)}
      {...props}
    >
      <path fill="#ff69b4" d="M0 0h4v4H0z" />
      <path d="M4 0h4v4H4z" />
      <path fill="#ff69b4" d="M8 0h4v4H8z" />
      <path d="M12 0h4v4h-4zM0 4h4v4H0z" />
      <path fill="#ff69b4" d="M4 4h4v4H4z" />
      <path d="M8 4h4v4H8z" />
      <path fill="#ff69b4" d="M12 4h4v4h-4zM0 8h4v4H0z" />
      <path d="M4 8h4v4H4z" />
      <path fill="#ff69b4" d="M8 8h4v4H8z" />
      <path d="M12 8h4v4h-4zM0 12h4v4H0z" />
      <path fill="#ff69b4" d="M4 12h4v4H4z" />
      <path d="M8 12h4v4H8z" />
      <path fill="#ff69b4" d="M12 12h4v4h-4z" />
    </svg>
  );
};
