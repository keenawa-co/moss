import { ComponentPropsWithoutRef, createContext, ElementRef, forwardRef, useContext } from "react";

import * as RadioGroupPrimitive from "@radix-ui/react-radio-group";
import { cn } from "@repo/moss-ui";
import { radio, type RadioProps } from "@tailus/themer";

export interface RadioRootProps extends RadioProps {
  className?: string;
}

const RadioGroupContext = createContext<RadioRootProps>({ fancy: false, intent: "primary" });

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

const defaultRadioGroupItemStyles = `size-4 border bg-white shadow-sm group rounded-full peer flex justify-center items-center outline-2 outline-blue-600 outline-offset-2
 hover:brightness-95
 focus-visible:outline
 bg-gray-500/10
 data-[state=checked]:border-none
 data-[state=checked]:bg-blue-600

 disabled:bg-gray-800
 disabled:opacity-50
 disabled:border-gray-700
 disabled:shadow-none
 disabled:cursor-not-allowed
 disabled:data-[state=checked]:bg-gray-300
 disabled:data-[state=checked]:bg-gray-700
 disabled:data-[state=checked]:shadow-none
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

const Indicator = forwardRef<
  ElementRef<typeof RadioGroupPrimitive.Indicator>,
  ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Indicator> &
    RadioProps & {
      className?: string;
    }
>((props, forwardedRef) => {
  const { intent } = useContext(RadioGroupContext);
  const { indicator } = radio({ intent });
  return (
    <RadioGroupPrimitive.Indicator
      {...props}
      ref={forwardedRef}
      className={indicator({ intent: props.intent, className: props.className })}
    />
  );
});

export { Root, Item, Indicator };
