import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as CheckboxPrimitive from "@radix-ui/react-checkbox";

export interface CheckboxProps {
  className?: string;
}

const defaultCheckboxRootStyles = `border-1 border-solid border-[rgb(228,228,231)] dark:border-[rgb(39,39,42)] group rounded peer flex justify-center items-center size-[1.125rem] text-white
  focus-visible:outline-2
  focus-visible:outline-bg-[rgb(37,99,235)]
  focus-visible:outline-offset-2
  focus-visible:outline

  hover:brightness-95

  disabled:bg-gray-100
  disabled:opacity-50
  disabled:border-gray-300
  disabled:shadow-none
  disabled:cursor-not-allowed

  disabled:data-[state=checked]:bg-gray-300
  disabled:data-[state=checked]:shadow-none
  disabled:data-[state=indeterminate]:bg-gray-300
  disabled:data-[state=indeterminate]:shadow-none

  dark:bg-gray-500/10
  dark:disabled:bg-gray-800
  dark:disabled:border-gray-700
  dark:disabled:data-[state=checked]:bg-gray-700

  data-[state=checked]:border-none
  data-[state=checked]:bg-[rgb(37,99,235)]
  dark:data-[state=checked]:bg-[rgb(37,99,235)]
  data-[state=indeterminate]:bg-[rgb(37,99,235)]
  data-[state=indeterminate]:border-none
`;

const CheckboxRoot = forwardRef<
  ElementRef<typeof CheckboxPrimitive.Root>,
  ComponentPropsWithoutRef<typeof CheckboxPrimitive.Root> & CheckboxProps
>(({ className, ...props }: CheckboxProps, forwardedRef) => {
  return <CheckboxPrimitive.Root ref={forwardedRef} className={cn(defaultCheckboxRootStyles, className)} {...props} />;
});

const Root = CheckboxRoot;
const Indicator = CheckboxPrimitive.Indicator;

export { Root, Indicator };
