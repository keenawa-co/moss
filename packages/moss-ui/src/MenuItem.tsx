import { ComponentPropsWithoutRef } from "react";
import { twMerge } from "tailwind-merge";

export const MenuItem = ({ children, className }: { title?: string } & ComponentPropsWithoutRef<"div">) => {
  return (
    <div
      className={twMerge(
        "ml-3.5 flex h-8 items-center gap-2.5 rounded-lg pl-2.5 hover:bg-zinc-500 hover:bg-zinc-500/10 focus:bg-zinc-500 focus:bg-zinc-500/10",
        className
      )}
    >
      {children}
    </div>
  );
};

export default MenuItem;
