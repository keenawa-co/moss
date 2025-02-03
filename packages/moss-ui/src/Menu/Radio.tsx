import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import * as MenuPrimitive from "@radix-ui/react-menu";

import Icon from "../Icon";
import { cn } from "../utils";
import { ScopedProps } from "./types";

/* -------------------------------------------------------------------------------------------------
 * ContextMenuRadioGroup
 * -----------------------------------------------------------------------------------------------*/

export type RadioGroupElement = ElementRef<typeof MenuPrimitive.RadioGroup>;
export type RadioGroupProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.RadioGroup>>;

export const RadioGroup = forwardRef<RadioGroupElement, RadioGroupProps>(
  (props: ScopedProps<RadioGroupProps>, forwardedRef) => {
    return <MenuPrimitive.RadioGroup {...props} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * ContextMenuRadioItem
 * -----------------------------------------------------------------------------------------------*/

export type RadioItemElement = ElementRef<typeof MenuPrimitive.RadioItem>;
export type RadioItemProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.RadioItem>> & {
  label: string;
  checked: boolean;
  disabled?: boolean;
};

export const RadioItem = forwardRef<RadioItemElement, RadioItemProps>(
  (props: ScopedProps<RadioItemProps>, forwardedRef) => {
    return (
      <MenuPrimitive.RadioItem
        {...props}
        ref={forwardedRef}
        className={cn("flex items-center gap-1.5 rounded px-2 py-1", {
          "cursor-not-allowed opacity-50": props.disabled,
          "cursor-pointer hover:bg-[#D4E2FF] hover:outline-hidden": !props.disabled,
        })}
      >
        {props.checked ? (
          <Icon icon="DropdownMenuRadioIndicator" className="size-2" />
        ) : (
          <Icon icon="DropdownMenuRadioIndicator" className="size-2 opacity-0" />
        )}

        <div className="flex w-full items-center gap-2.5">
          <span>{props.label}</span>
        </div>
      </MenuPrimitive.RadioItem>
    );
  }
);
