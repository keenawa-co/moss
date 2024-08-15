import { ComponentPropsWithoutRef } from "react";
import * as icons from "../../icons/build";
import { cn } from "./lib/utils";

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

  return <IconTag className={cn("group-text-zinc-500 group-hover:text-zinc-600", className)} {...props} />;
};

export default Icon;
