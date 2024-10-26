import Icon, { Icons } from "@/src/Icon";
import { cn } from "@/src/utils/utils";
import { Scope } from "@radix-ui/react-context";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { ElementRef, ComponentPropsWithoutRef, forwardRef } from "react";
import { ScopedProps } from "./types";

export type ItemElement = ElementRef<typeof MenuPrimitive.Item>;
export type ItemProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.Item>> & {
  label: string;
  shortcut?: string[];
  disabled?: boolean;
  icon?: Icons;
  hideIcon?: boolean;
};

export const Item = forwardRef<ItemElement, ItemProps>(({ hideIcon = false, ...props }, forwardedRef) => {
  return (
    <MenuPrimitive.Item
      {...props}
      ref={forwardedRef}
      className={cn("flex items-center gap-1.5 rounded px-2 py-1", {
        "cursor-not-allowed opacity-50": props.disabled,
        "cursor-pointer hover:bg-[#D4E2FF] hover:outline-none": !props.disabled,
      })}
    >
      {!hideIcon && (props.icon ? <Icon icon={props.icon} /> : <Icon icon="Documentation" className="opacity-0" />)}

      <span className="font-medium">{props.label}</span>

      {props.shortcut && <div className="ml-auto">{props.shortcut.join("")}</div>}
    </MenuPrimitive.Item>
  );
});
