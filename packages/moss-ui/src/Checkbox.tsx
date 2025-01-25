import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import * as CheckboxPrimitive from "@radix-ui/react-checkbox";
import { checkbox, fancyCheckbox, type CheckboxProps as CheckboxVariants } from "@tailus/themer";

export interface CheckboxProps extends CheckboxVariants {
  className?: string;
  fancy?: boolean;
}

const CheckboxRoot = forwardRef<
  ElementRef<typeof CheckboxPrimitive.Root>,
  ComponentPropsWithoutRef<typeof CheckboxPrimitive.Root> & CheckboxProps
>(({ className, intent, fancy, ...props }: CheckboxProps, forwardedRef) => {
  const classes = fancy ? fancyCheckbox({ intent, className }) : checkbox({ intent, className });
  return <CheckboxPrimitive.Root ref={forwardedRef} className={classes} {...props} />;
});

const CheckboxIndicator = CheckboxPrimitive.Indicator;

export { CheckboxRoot, CheckboxIndicator };

export default {
  Root: CheckboxRoot,
  Indicator: CheckboxIndicator,
};
