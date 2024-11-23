import { ComponentProps } from "react";
import { twMerge } from "tailwind-merge";
import { HeadBar } from "../HeadBar/HeadBar";
import StatusBar from "../StatusBar";

const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <div className="grid h-full grid-rows-[minmax(0px,46px)_1fr_auto] background-[var(--color-page-background)]">
      <HeadBar />

      <main className={twMerge("background-[var(--color-page-background)]", className)} {...props}>
        {children}
      </main>

      <StatusBar className="h-5.5 w-full" branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri" />
    </div>
  );
};

export default RootLayout;
