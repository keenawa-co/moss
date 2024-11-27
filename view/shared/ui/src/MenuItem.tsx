import { twMerge } from "tailwind-merge";
import { ComponentPropsWithoutRef } from "react";

export const MenuItem = ({ children, className }: { title?: string } & ComponentPropsWithoutRef<"div">) => {
  return (
    <div
      className={twMerge(
        "hover:bg-zinc-500 focus:bg-zinc-500 ml-3.5 flex h-8 items-center gap-2.5 rounded-lg pl-2.5  hover:bg-opacity-10 focus:bg-opacity-10",
        className
      )}
    >
      {children}
    </div>
  );
};

export default MenuItem;
