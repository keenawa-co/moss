import { cva } from "class-variance-authority";
import { Children, forwardRef, HTMLAttributes, isValidElement } from "react";

import { cn, Icon } from "@repo/moss-ui";

export type Root = typeof Root;
export type Label = typeof Label;

export interface ButtonProps extends HTMLAttributes<HTMLButtonElement | HTMLAnchorElement> {
  loading?: boolean;
  disabled?: boolean;
  href?: string;
  intent?: "primary" | "danger" | "warning" | "success" | "neutral";
  variant?: "solid" | "outlined" | "soft" | "ghost";
  size?: "xs" | "sm" | "md" | "lg" | "xl";
}

const buttonRootStyles = cva(
  "relative flex items-center justify-center rounded transition-colors focus:outline-none focus:ring-2 ",
  {
    variants: {
      intent: {
        primary: "[--bg:#0073ca] [--bg-hover:#0c92eb] [--border:#0073ca] [--text:white] [--ring:#b9e0fe]",
        warning: "[--bg:#d1bf00] [--bg-hover:#ffff00] [--border:#d1bf00] [--text:white] [--ring:#eeff86]",
        success: "[--bg:#53b800] [--bg-hover:#6ee600] [--border:#53b800] [--text:white] [--ring:#d0ff90]",
        danger: "[--bg:#ff0000] [--bg-hover:#ff5757] [--border:#ff0000] [--text:white] [--ring:#ffc0c0]",
        neutral: "[--bg:#969696] [--bg-hover:#aaaaaa] [--border:#969696] [--text:white] [--ring:#e3e3e3]",
      },
      variant: {
        solid:
          "background-[--bg] inset-border border-[--border] hover:background-[--bg-hover] text-[--text] focus:ring-[--ring]",
        outlined: "inset-border border-[--border] background-[--ring] text-[--bg] focus:ring-[--border] ",
        soft: "background-[--bg-hover] text-[--text]  focus:ring-[--ring]",
        ghost: "text-[--bg] hover:background-[--ring] focus:ring-transparent",
      },
      size: {
        "xs": "h-7 px-3",
        "sm": "h-8 px-3.5",
        "md": "h-9 px-4",
        "lg": "h-10 px-5",
        "xl": "h-12 px-6",
      },
      disabled: {
        false: null,
        true: ["opacity-40", "cursor-not-allowed"],
      },
      loading: {
        false: null,
        true: ["[&>:not(.LoadingIcon)]:opacity-0", "cursor-progress "],
      },
    },
    compoundVariants: [],
  }
);

export const Label = forwardRef<HTMLElement, HTMLAttributes<HTMLElement>>(
  ({ className, children, ...props }, forwardedRef) => {
    return (
      <span className={className} {...props} ref={forwardedRef}>
        {children}
      </span>
    );
  }
);

export const Root = forwardRef<HTMLButtonElement & HTMLAnchorElement, ButtonProps>(
  (
    { className, variant = "solid", size = "md", disabled, loading, href, children, intent = "primary", ...props },
    forwardedRef
  ) => {
    const Component = href ? "a" : "button";
    const iconOnly = Children.toArray(children).some(
      (child) => isValidElement(child) && child.type === Icon && child.props.type === "only"
    );
    const buttonSize = iconOnly ? "iconOnlyButtonSize" : "size";

    return (
      <Component
        ref={forwardedRef}
        href={href}
        className={cn(buttonRootStyles({ intent, variant, size, disabled, loading, className }), buttonSize)}
        disabled={disabled || loading}
        {...props}
      >
        {children}

        {loading && (
          <div className="LoadingIcon absolute inset-0 grid place-items-center">
            <Icon icon="Loader" className="animate-spin" />
          </div>
        )}
      </Component>
    );
  }
);

export default { Root, Label };
