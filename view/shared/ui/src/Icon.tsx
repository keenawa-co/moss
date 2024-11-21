import { ComponentPropsWithoutRef } from "react";
import * as icons from "@repo/icongen";
import { cn } from "./utils/utils";

export type Icons = keyof typeof icons;

export const Icon = ({
  icon,
  className,
  ...props
}: {
  icon: Icons | null;
  className?: string;
} & ComponentPropsWithoutRef<"svg">) => {
  const IconTag = icons[icon];

  if (!IconTag) {
    return (
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" className={cn("size-4 flex-shrink-0", className)}>
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
  }

  return <IconTag className={cn("size-4 flex-shrink-0 text-[rgba(var(--colorPrimary))]", className)} {...props} />;
};

export default Icon;
