import { ComponentPropsWithoutRef, createContext, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as RadioGroupPrimitive from "@radix-ui/react-radio-group";

export interface RadioRootProps {
  className?: string;
}

const RadioGroupContext = createContext<RadioRootProps>({});

const Root = forwardRef<
  ElementRef<typeof RadioGroupPrimitive.Root>,
  ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Root> & RadioRootProps
>(({ className, ...props }, forwardedRef) => {
  return (
    <RadioGroupContext.Provider value={{}}>
      <RadioGroupPrimitive.Root {...props} ref={forwardedRef} className={className} />
    </RadioGroupContext.Provider>
  );
});

export interface RadioItemProps {
  className?: string;
}

const defaultRadioGroupItemStyles = `
  flex justify-center items-center cursor-pointer rounded-full size-[18px] group
  bg-white border-1 border-solid border-[rgb(228,228,231)] dark:border-[rgb(39,39,42)]

  focus-visible:outline-2 focus-visible:outline-[rgb(37,99,235)] focus-visible:outline-offset-2
  hover:brightness-95

  disabled:bg-gray-100 disabled:opacity-50 disabled:border-gray-300
  disabled:data-[state=checked]:bg-gray-300

  dark:bg-gray-500/10
  dark:disabled:bg-gray-800 dark:disabled:border-gray-700
  dark:disabled:data-[state=checked]:bg-gray-700

  data-[state=checked]:bg-[rgb(37,99,235)] dark:data-[state=checked]:bg-[rgb(37,99,235)]
  data-[state=checked]:border-none
`;

const Item = forwardRef<
  ElementRef<typeof RadioGroupPrimitive.Item>,
  ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Item> & RadioItemProps
>((props, forwardedRef) => {
  return (
    <RadioGroupPrimitive.Item
      {...props}
      ref={forwardedRef}
      className={cn(defaultRadioGroupItemStyles, props.className)}
    />
  );
});
const defaultRadioGroupIndicatorStyles = ` *:size-[10px]`;
const Indicator = forwardRef<
  ElementRef<typeof RadioGroupPrimitive.Indicator>,
  ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Indicator> & {
    className?: string;
  }
>((props, forwardedRef) => {
  return (
    <RadioGroupPrimitive.Indicator
      {...props}
      ref={forwardedRef}
      className={cn(defaultRadioGroupIndicatorStyles, props.className)}
    />
  );
});

export { Root, Item, Indicator };
