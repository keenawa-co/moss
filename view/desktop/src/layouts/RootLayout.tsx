import { ComponentProps } from "react";

import { cn } from "@repo/moss-ui";

import { HeadBar } from "../parts/HeadBar/HeadBar";
import StatusBar from "../parts/StatusBar/StatusBar";

export const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <div className="grid h-full grid-rows-[minmax(0px,46px)_1fr_auto] background-[var(--moss-page-background)]">
      <HeadBar />

      <main className={cn("background-[var(--moss-page-background)]", className)} {...props}>
        {children}
      </main>

      <StatusBar className="h-5.5 w-full" />
    </div>
  );
};
