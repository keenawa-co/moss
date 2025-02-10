import { ComponentProps } from "react";

import { cn } from "@/utils";

import { HeadBar } from "../parts/HeadBar/HeadBar";
import StatusBar from "../parts/StatusBar/StatusBar";

export const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <div className="background-(--moss-page-background) grid h-full grid-rows-[minmax(0px,46px)_1fr_auto]">
      <HeadBar />

      <main className={cn("background-(--moss-page-background)", className)} {...props}>
        {children}
      </main>

      <StatusBar className="h-5.5 w-full" />
    </div>
  );
};
