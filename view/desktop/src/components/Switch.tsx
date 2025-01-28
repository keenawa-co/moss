import { ComponentPropsWithoutRef, createContext, ElementRef, forwardRef } from "react";

import * as Switch from "@radix-ui/react-switch";
import { cn } from "@repo/moss-ui";
import { type SwitchProps } from "@tailus/themer";

const SwitchContext = createContext({});

const defaultSwitchRootStyles = `relative h-5 w-8 inline-block border border-gray-900/5 group rounded-full bg-gray-200 transition outline-2 outline-blue-600 outline-offset-2 overflow-hidden
  focus-visible:outline
  disabled:bg-gray-700
  disabled:opacity-50
  disabled:shadow-none
  disabled:cursor-not-allowed
  data-[state=checked]:border-blue-600
  data-[state=checked]:bg-blue-600
`;

const Root = forwardRef<ElementRef<typeof Switch.Root>, ComponentPropsWithoutRef<typeof Switch.Root>>(
  ({ className, ...props }, forwardedRef) => {
    return (
      <SwitchContext.Provider value={{}}>
        <Switch.Root className={cn(defaultSwitchRootStyles, className)} {...props} ref={forwardedRef} />
      </SwitchContext.Provider>
    );
  }
);

const defaultSwitchThumbStyles = `absolute inset-x-[1px] inset-y-0 my-auto size-4 rounded-full bg-white shadow-sm shadow-gray-950/25 transition-[transform,width] ease-in-out duration-300 will-change-transform
   data-[state=checked]:translate-x-3`;

const Thumb = forwardRef<ElementRef<typeof Switch.Thumb>, ComponentPropsWithoutRef<typeof Switch.Thumb> & SwitchProps>(
  ({ className, ...props }, forwardedRef) => {
    return <Switch.Thumb className={cn(defaultSwitchThumbStyles, className)} {...props} ref={forwardedRef} />;
  }
);

export { Root, Thumb };
