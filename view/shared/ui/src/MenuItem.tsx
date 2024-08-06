import { twMerge } from "tailwind-merge";
import { ComponentPropsWithoutRef } from "react";

export const MenuItem = ({ children, title, className }: { title?: string } & ComponentPropsWithoutRef<"div">) => {
  return (
    <div
      className={twMerge(
        "flex items-center gap-2.5 hover:bg-zinc-500 hover:bg-opacity-10 focus:bg-zinc-500 focus:bg-opacity-10 rounded-lg transition ml-3.5 w-57 h-8 pl-2.5",
        className
      )}
    >
      {children}
    </div>
  );
};

export default MenuItem;
