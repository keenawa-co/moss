import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";

import Icon, { Icons } from "../Icon";
import { ScopedProps } from "./types";

export type ItemElement = ElementRef<typeof MenuPrimitive.Item>;
export type ItemProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.Item>> & {
  label: string;
  shortcut?: string[];
  disabled?: boolean;
  icon?: Icons;
  hideIcon?: boolean;
  iconClassName?: string;
};

export const Item = forwardRef<ItemElement, ItemProps>(
  ({ iconClassName, className, hideIcon = false, ...props }, forwardedRef) => {
    return (
      <MenuPrimitive.Item
        {...props}
        ref={forwardedRef}
        className={cn(
          "flex items-center gap-1.5 rounded py-0.5 pr-5 pl-[7px]",
          {
            "cursor-not-allowed opacity-50": props.disabled,
            "cursor-pointer hover:bg-[#D4E2FF] hover:outline-hidden": !props.disabled,
          },
          className
        )}
      >
        {!hideIcon &&
          (props.icon ? (
            <Icon icon={props.icon} className={cn("shrink-0 text-[#8D8D8D]", iconClassName)} />
          ) : (
            <Icon icon="Documentation" className={cn("shrink-0 opacity-0", iconClassName)} />
          ))}
        <div className="flex w-full items-center gap-2.5">
          <span>{props.label}</span>

          {props.shortcut && <div className="ml-auto text-[#8D8D8D]">{props.shortcut.join("")}</div>}
        </div>
      </MenuPrimitive.Item>
    );
  }
);
