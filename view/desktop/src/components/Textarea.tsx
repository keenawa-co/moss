import { cva } from "class-variance-authority";
import { forwardRef, RefObject, TextareaHTMLAttributes } from "react";

import { cn } from "@/utils";

export interface TextAreaProps extends Omit<TextareaHTMLAttributes<HTMLTextAreaElement>, "size"> {
  variant?: "plain" | "soft" | "outlined" | "mixed" | "bottomOutlined";
}

const textareaVariants = cva(
  "w-full peer placeholder-[rgb(161,161,170)] dark:placeholder-[rgb(82,82,91)] text-[rgb(9,9,11)] dark:text-white py-2  px-3",
  {
    variants: {
      variant: {
        plain: `
          rounded-sm
          background-(--moss-textarea-bg-plain)
          transition-[outline] outline-none
          data-[invalid]:text-[rgb(220,38,38)]   dark:data-[invalid]:text-[rgb(248,113,113)]
          data-[valid]:text-[rgba(22,101,52,25)] dark:data-[valid]:text-[rgb(220,252,231)]
        `,
        soft: `
          rounded-sm
          background-(--moss-textarea-bg-soft)
          transition-[outline] outline-none focus:brightness-95 dark:focus:brightness-105
          data-[invalid]:bg-[rgb(254,226,226)] dark:data-[invalid]:bg-[rgba(153,27,27,25)]
          data-[valid]:bg-[rgb(220,252,231)]   dark:data-[valid]:bg-[rgba(22,101,52,25)]
        `,
        outlined: `
          rounded-sm
          background-(--moss-textarea-bg-outlined)
          transition-[outline] focus:outline-2 focus:outline-[rgb(37,99,235)] -outline-offset-1
          border border-(--moss-textarea-border-outlined)
               data-[valid]:border-[rgb(22,163,74)]        focus:data-[valid]:outline-[rgb(22,163,74)]
          dark:data-[valid]:border-[rgb(34,197,94)]   dark:focus:data-[valid]:outline-[rgb(34,197,94)]
               data-[invalid]:border-[rgb(220,38,38)]      focus:data-[invalid]:outline-[rgb(220,38,38)]
          dark:data-[invalid]:border-[rgb(239,68,68)] dark:focus:data-[invalid]:outline-[rgb(239,68,68)]
        `,
        mixed: `
        rounded-sm
        border border-(--moss-textarea-border-mixed)
        background-(--moss-textarea-bg-mixed)
        shadow-sm shadow-gray-900/5 dark:shadow-gray-900/35
        transition-[outline] -outline-offset-1 focus:outline-2 focus:outline-[rgb(37,99,235)]
             data-[invalid]:border-[rgb(220,38,38)]      focus:data-[invalid]:outline-[rgb(220,38,38)]
        dark:data-[invalid]:border-[rgb(239,68,68)] dark:focus:data-[invalid]:outline-[rgb(239,68,68)]
             data-[valid]:border-[rgb(22,163,74)]      focus:data-[valid]:outline-[rgb(22,163,74)]
        dark:data-[valid]:border-[rgb(34,197,94)] dark:focus:data-[valid]:outline-[rgb(34,197,94)]
      `,
        bottomOutlined: `
          rounded-none
          background-(--moss-textarea-bg-bottomOutlined)
          transition-[border] focus:outline-none
          border-b border-(--moss-textarea-border-bottomOutlined)
          focus:border-b-2 focus:border-[rgb(37,99,235)]
          data-[invalid]:border-[rgb(248,113,113)] dark:data-[invalid]:border-[rgb(220,38,38)]
          data-[valid]:border-[rgb(74,222,128)] dark:data-[valid]:border-[rgb(22,163,74)]
        `,
      },
      disabled: {
        false: null,
        true: "cursor-not-allowed opacity-50 active:pointer-events-none",
      },
    },
  }
);

export const Textarea = forwardRef<HTMLTextAreaElement, TextAreaProps>(
  ({ className, variant = "mixed", ...props }, forwardedRef) => {
    return (
      <textarea
        ref={forwardedRef as RefObject<HTMLTextAreaElement>}
        className={cn(textareaVariants({ variant, className }), className)}
        {...props}
      />
    );
  }
);

export default Textarea;
