import { cva } from "class-variance-authority";
import { Children, forwardRef, HTMLAttributes, isValidElement } from "react";

import Icon from "./Icon";
import { Background, Border, Effects, Spacing, Typography } from "./types";
import { cn } from "./utils";
import { toCssVarIfNecessary } from "./utils/toCssVarIfNecessary";

export type Root = typeof Root;
export type Label = typeof Label;

export interface ButtonStyleProps extends Background, Border, Spacing, Typography, Effects {}

export interface ButtonProps extends HTMLAttributes<HTMLButtonElement | HTMLAnchorElement> {
  loading?: boolean;
  disabled?: boolean;
  href?: string;
  variant?: "solid" | "outlined" | "soft" | "ghost";
  size?: "xs" | "sm" | "md" | "lg" | "xl";
  styles: ButtonStyleProps;
}

const buttonRootStyles = cva(
  "relative flex items-center justify-center rounded transition-colors focus:outline-none focus:ring-2 ",
  {
    variants: {
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
        true: ["opacity-40 pointer-events-none cursor-not-allowed"],
      },
      loading: {
        false: null,
        true: ["[&>:not(.LoadingIcon)]:opacity-0 pointer-events-none cursor-not-allowed"],
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
    { className, variant = "solid", size = "md", disabled, loading, href, children, styles, ...props },
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
        className={cn(buttonRootStyles({ variant, size, disabled, loading, className }), buttonSize)}
        disabled={disabled}
        {...props}
        style={
          {
            "--bg": toCssVarIfNecessary(styles?.background.default),
            "--bg-hover": toCssVarIfNecessary(styles?.background.hover),
            "--border": toCssVarIfNecessary(styles?.borderColor?.default),
            "--text": toCssVarIfNecessary(styles?.color?.default),
            "--ring": toCssVarIfNecessary(styles?.ring),
          } as React.CSSProperties
        }
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
