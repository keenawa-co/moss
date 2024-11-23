import { ComponentProps, forwardRef } from "react";
import { twMerge } from "tailwind-merge";

export const SidebarLayout = ({ className, children, ...props }: ComponentProps<"aside">) => {
  return (
    <aside
      className={twMerge("mb-5.5 flex flex-col bg-[rgba(var(--color-side-bar-background))]", className)}
      {...props}
    >
      {children}
    </aside>
  );
};

export const ContentLayout = forwardRef<HTMLDivElement, ComponentProps<"div">>(
  ({ children, className, ...props }, ref) => (
    <div ref={ref} className={twMerge("bg-bgPrimary mb-5.5 flex-1 overflow-auto", className)} {...props}>
      {children}
    </div>
  )
);

export const PropertiesLayout = ({ className, children, ...props }: ComponentProps<"aside">) => {
  return (
    <aside className={twMerge("h-[100vh + 10px] bg-bgPrimary mt-8 w-[50px] overflow-auto", className)} {...props}>
      {children}
    </aside>
  );
};

ContentLayout.displayName = "Content";
