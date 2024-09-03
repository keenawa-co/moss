import { ComponentProps, forwardRef } from "react";
import { twMerge } from "tailwind-merge";

export const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <main className={twMerge("flex flex-row h-screen bg-page-background", className)} {...props}>
      {children}
    </main>
  );
};

export const Sidebar = ({ className, children, ...props }: ComponentProps<"aside">) => {
  return (
    <aside
      className={twMerge("flex flex-col mt-11 mb-5.5 bg-sidebarBackground", className)}
      //className={twMerge('w-[200px] mt-8 h-[100vh + 10px] overflow-auto bg-bgPrimary', className)}
      {...props}
    >
      {children}
    </aside>
  );
};

export const Content = forwardRef<HTMLDivElement, ComponentProps<"div">>(({ children, className, ...props }, ref) => (
  <div ref={ref} className={twMerge("mt-11 mb-5.5 flex-1 overflow-auto bg-bgPrimary", className)} {...props}>
    {children}
  </div>
));

export const Properties = ({ className, children, ...props }: ComponentProps<"aside">) => {
  return (
    <aside className={twMerge("w-[50px] mt-8 h-[100vh + 10px] overflow-auto bg-bgPrimary", className)} {...props}>
      {children}
    </aside>
  );
};

Content.displayName = "Content";
