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
  "relative flex items-center cursor-pointer justify-center rounded-lg transition duration-150 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2 outline-blue-600",
  {
    variants: {
      intent: {
        primary: `[--bg-solid:var(--color-button-primary-solid-background)] [--border-solid:var(--color-button-primary-solid-border)] [--text-solid:var(--color-button-primary-solid-text)] [--bg-outlined:var(--color-button-primary-outlined-background)] [--border-outlined:var(--color-button-primary-outlined-border)] [--text-outlined:var(--color-button-primary-outlined-text)] [--bg-soft:var(--color-button-primary-soft-background)] [--border-soft:var(--color-button-primary-soft-border)] [--text-soft:var(--color-button-primary-soft-text)] [--bg-ghost:var(--color-button-primary-ghost-background)] [--border-ghost:var(--color-button-primary-ghost-border)] [--text-ghost:var(--color-button-primary-ghost-text)]`,
        warning: `[--bg-solid:var(--color-button-warning-solid-background)] [--border-solid:var(--color-button-warning-solid-border)] [--text-solid:var(--color-button-warning-solid-text)] [--bg-outlined:var(--color-button-warning-outlined-background)] [--border-outlined:var(--color-button-warning-outlined-border)] [--text-outlined:var(--color-button-warning-outlined-text)] [--bg-soft:var(--color-button-warning-soft-background)] [--border-soft:var(--color-button-warning-soft-border)] [--text-soft:var(--color-button-warning-soft-text)] [--bg-ghost:var(--color-button-warning-ghost-background)] [--border-ghost:var(--color-button-warning-ghost-border)] [--text-ghost:var(--color-button-warning-ghost-text)]`,
        success: `[--bg-solid:var(--color-button-success-solid-background)] [--border-solid:var(--color-button-success-solid-border)] [--text-solid:var(--color-button-success-solid-text)] [--bg-outlined:var(--color-button-success-outlined-background)] [--border-outlined:var(--color-button-success-outlined-border)] [--text-outlined:var(--color-button-success-outlined-text)] [--bg-soft:var(--color-button-success-soft-background)] [--border-soft:var(--color-button-success-soft-border)] [--text-soft:var(--color-button-success-soft-text)] [--bg-ghost:var(--color-button-success-ghost-background)] [--border-ghost:var(--color-button-success-ghost-border)] [--text-ghost:var(--color-button-success-ghost-text)]`,
        danger: ` [--bg-solid:var(--color-button-danger-solid-background)]  [--border-solid:var(--color-button-danger-solid-border)]  [--text-solid:var(--color-button-danger-solid-text)]  [--bg-outlined:var(--color-button-danger-outlined-background)]  [--border-outlined:var(--color-button-danger-outlined-border)]  [--text-outlined:var(--color-button-danger-outlined-text)]  [--bg-soft:var(--color-button-danger-soft-background)]  [--border-soft:var(--color-button-danger-soft-border)]  [--text-soft:var(--color-button-danger-soft-text)]  [--bg-ghost:var(--color-button-danger-ghost-background)]  [--border-ghost:var(--color-button-danger-ghost-border)]  [--text-ghost:var(--color-button-danger-ghost-text)]`,
        neutral: `[--bg-solid:var(--color-button-neutral-solid-background)] [--border-solid:var(--color-button-neutral-solid-border)] [--text-solid:var(--color-button-neutral-solid-text)] [--bg-outlined:var(--color-button-neutral-outlined-background)] [--border-outlined:var(--color-button-neutral-outlined-border)] [--text-outlined:var(--color-button-neutral-outlined-text)] [--bg-soft:var(--color-button-neutral-soft-background)] [--border-soft:var(--color-button-neutral-soft-border)] [--text-soft:var(--color-button-neutral-soft-text)] [--bg-ghost:var(--color-button-neutral-ghost-background)] [--border-ghost:var(--color-button-neutral-ghost-border)] [--text-ghost:var(--color-button-neutral-ghost-text)]`,
      },
      variant: {
        solid: `   background-(--bg-solid)    text-(--text-solid)    [box-shadow:rgba(255,255,255,0.25)_0px_1px_0px_0px_inset,var(--border-solid)_0px_0px_0px_1px] hover:brightness-110 active:brightness-95`,
        outlined: `background-(--bg-outlined) text-(--text-outlined) [box-shadow:rgba(255,255,255,0.25)_0px_1px_0px_0px_inset,var(--border-outlined)_0px_0px_0px_1px] hover:brightness-[0.98]  active:brightness-100`,
        soft: `    background-(--bg-soft)     text-(--text-soft)     [box-shadow:var(--border-soft)_0px_0px_0px_1px] hover:brightness-95 active:brightness-105`,
        ghost: `   background-transparent     text-(--text-ghost)    hover:background-(--bg-ghost) hover:[box-shadow:var(--border-ghost)_0px_0px_0px_1px] active:brightness-95`,
      },
      size: {
        "xs": "h-7",
        "sm": "h-8",
        "md": "h-9",
        "lg": "h-10",
        "xl": "h-12",
      },
      disabled: {
        false: null,
        true: "grayscale-70 cursor-not-allowed hover:brightness-100 active:brightness-100 active:pointer-events-none",
      },
      loading: {
        false: null,
        true: "[&>:not(.LoadingIcon)]:opacity-0 cursor-progress",
      },
      Component: {
        a: "max-w-max",
        button: null,
      },
      iconOnly: {
        false: "notOnlyIcon",
        true: "iconOnly",
      },
    },
    compoundVariants: [
      {
        iconOnly: true,
        size: "xs",
        className: "px-1.5",
      },
      {
        iconOnly: false,
        size: "xs",
        className: "px-3",
      },
      {
        iconOnly: true,
        size: "sm",
        className: "px-2",
      },
      {
        iconOnly: false,
        size: "sm",
        className: "px-3.5",
      },

      {
        iconOnly: true,
        size: "md",
        className: "px-2.5",
      },
      {
        iconOnly: false,
        size: "md",
        className: " px-4",
      },
      {
        iconOnly: true,
        size: "lg",
        className: "px-3",
      },
      {
        iconOnly: false,
        size: "lg",
        className: " px-5",
      },
      {
        iconOnly: true,
        size: "xl",
        className: "px-4",
      },
      {
        iconOnly: false,
        size: "xl",
        className: "px-6",
      },
    ],
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
    const iconOnly =
      Children.toArray(children).length === 1 &&
      Children.toArray(children).some((child) => isValidElement(child) && child.type === Icon);

    return (
      <Component
        ref={forwardedRef}
        href={href}
        className={cn(buttonRootStyles({ intent, variant, size, disabled, loading, className, Component, iconOnly }))}
        disabled={disabled || loading}
        {...props}
      >
        {children}

        {loading && (
          <div className="LoadingIcon absolute inset-0 grid place-items-center">
            <Icon icon="LoaderTailus" className="animate-spin" />
          </div>
        )}
      </Component>
    );
  }
);

export default { Root, Label };
