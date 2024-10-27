import Icon, { Icons } from "@/src/Icon";
import { cn } from "@/src/utils/utils";
import { Scope } from "@radix-ui/react-context";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { ElementRef, ComponentPropsWithoutRef, forwardRef } from "react";
import { ScopedProps } from "./types";

/* -------------------------------------------------------------------------------------------------
 * SubTrigger
 * -----------------------------------------------------------------------------------------------*/

export type SubTriggerElement = ElementRef<typeof MenuPrimitive.SubTrigger>;
export type SubTriggerProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.SubTrigger>> & {
  label: string;
  icon?: Icons;
  hideIcon?: boolean;
};

export const SubTrigger = forwardRef<SubTriggerElement, SubTriggerProps>(
  ({ hideIcon = false, ...props }, forwardedRef) => {
    return (
      <MenuPrimitive.SubTrigger
        {...props}
        ref={forwardedRef}
        className={cn("flex items-center gap-1.5 rounded px-2 py-1", {
          "cursor-not-allowed opacity-50": props.disabled,
          "cursor-pointer hover:bg-[#D4E2FF] hover:outline-none": !props.disabled,
        })}
      >
        {!hideIcon &&
          (props.icon ? (
            <Icon icon={props.icon} className="text-[#8D8D8D]" />
          ) : (
            <Icon icon="Documentation" className="opacity-0" />
          ))}

        <span>{props.label}</span>

        <Icon icon="ArrowheadRight" className="ml-auto text-[#8D8D8D]" />
      </MenuPrimitive.SubTrigger>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * SubContent
 * -----------------------------------------------------------------------------------------------*/

export type SubContentElement = ElementRef<typeof MenuPrimitive.Content>;
export type SubContentProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.SubContent>>;

export const SubContent = forwardRef<SubContentElement, SubContentProps>(
  (props: ScopedProps<SubContentProps>, forwardedRef) => {
    return (
      <MenuPrimitive.SubContent
        {...props}
        ref={forwardedRef}
        sideOffset={16}
        style={{ ...props.style }}
        className={cn("rounded-lg bg-white px-3 py-2 shadow-lg", props.className)}
      />
    );
  }
);
