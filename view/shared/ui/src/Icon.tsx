import { twMerge } from "tailwind-merge";
import { ComponentPropsWithoutRef } from "react";

export const Icon = ({ className, children, ...props }: ComponentPropsWithoutRef<"div">) => {
  return (
    <div
      className={twMerge("opacity-80 group-hover:opacity-100 group-focus:opacity-100 transition", className)}
      {...props}
    >
      {children}
    </div>
  );
};

export default Icon;
