import { ComponentPropsWithoutRef } from "react";
import * as icons from "../../icons/build";
import { cn } from "./utils/utils";

export type Icons = keyof typeof icons;

export const Icon = ({
  icon,
  className,
  ...props
}: {
  icon: Icons;
  className?: string;
} & ComponentPropsWithoutRef<"svg">) => {
  const IconTag = icons[icon];

  return <IconTag className={cn("text-[rgba(var(--color-primary))] hover:text-zinc-600", className)} {...props} />;
};

export default Icon;
