import { cva } from "class-variance-authority";
import React from "react";

import { cn } from "@/utils";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  variant?: "plain" | "soft" | "outlined" | "mixed" | "bottomOutlined";
  size?: "sm" | "md" | "lg" | "xl";
}

const inputVariants = cva(
  "w-full peer placeholder-[rgb(161,161,170)] dark:placeholder-[rgb(82,82,91)] text-[rgb(9,9,11)] dark:text-white",
  {
    variants: {
      variant: {
        plain: `
          py-0 rounded-md
          background-(--moss-input-bg-plain)
          transition-[outline] outline-none
          data-[invalid]:text-[rgb(220,38,38)]   dark:data-[invalid]:text-[rgb(248,113,113)]
          data-[valid]:text-[rgba(22,101,52,25)] dark:data-[valid]:text-[rgb(220,252,231)]
        `,
        soft: `
          rounded-md
          background-(--moss-input-bg-soft)
          transition-[outline] outline-none focus:brightness-95 dark:focus:brightness-105
          data-[invalid]:bg-[rgb(254,226,226)] dark:data-[invalid]:bg-[rgba(153,27,27,25)]
          data-[valid]:bg-[rgb(220,252,231)]   dark:data-[valid]:bg-[rgba(22,101,52,25)]
        `,
        outlined: `
          rounded-md
          background-(--moss-input-bg-outlined)
          transition-[outline] focus:outline-2 focus:outline-[rgb(37,99,235)] -outline-offset-1
          border border-(--moss-input-border-outlined)
               data-[valid]:border-[rgb(22,163,74)]      focus:data-[valid]:outline-[rgb(22,163,74)]
          dark:data-[valid]:border-[rgb(34,197,94)] dark:focus:data-[valid]:outline-[rgb(34,197,94)]
               data-[invalid]:border-[rgb(220,38,38)]      focus:data-[invalid]:outline-[rgb(220,38,38)]
          dark:data-[invalid]:border-[rgb(239,68,68)] dark:focus:data-[invalid]:outline-[rgb(239,68,68)]
        `,
        mixed: `
          rounded-md
          background-(--moss-input-bg-mixed)
          transition-[outline] focus:outline-2 focus:outline-[rgb(37,99,235)]
          shadow-sm shadow-gray-900/5 -outline-offset-1 border border-(--moss-input-border-mixed)
          dark:shadow-gray-900/35
          data-[invalid]:border-[rgb(220,38,38)] focus:data-[invalid]:outline-[rgb(220,38,38)]
          dark:data-[invalid]:border-[rgb(239,68,68)] dark:focus:data-[invalid]:outline-[rgb(239,68,68)]
          data-[valid]:border-[rgb(22,163,74)] focus:data-[valid]:outline-[rgb(22,163,74)]
          dark:data-[valid]:border-[rgb(34,197,94)] dark:focus:data-[valid]:outline-[rgb(34,197,94)]
        `,
        bottomOutlined: `
          rounded-none
          background-(--moss-input-bg-bottomOutlined)
          transition-[border] focus:outline-none
          border-b border-(--moss-input-border-bottomOutlined)
          focus:border-b-2 focus:border-[rgb(37,99,235)]
          data-[invalid]:border-[rgb(248,113,113)] dark:data-[invalid]:border-[rgb(220,38,38)]
          data-[valid]:border-[rgb(74,222,128)] dark:data-[valid]:border-[rgb(22,163,74)]
        `,
      },
      size: {
        sm: `text-[14px] h-8  px-2.5`,
        md: `text-[14px] h-9  px-3`,
        lg: `text-base   h-10 px-4`,
        xl: `text-base   h-12 px-5`,
      },
      disabled: {
        false: null,
        true: "cursor-not-allowed opacity-50 active:pointer-events-none",
      },
    },
  }
);

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ variant = "mixed", className, size = "md", disabled = false, ...props }, forwardedRef) => {
    return (
      <input
        ref={forwardedRef}
        className={cn(inputVariants({ variant, disabled, size }), className)}
        disabled={disabled}
        {...props}
      />
    );
  }
);

export default Input;
