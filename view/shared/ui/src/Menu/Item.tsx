import { cn } from "../utils/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";
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
          "flex items-center gap-1.5 rounded py-0.5 pl-[7px] pr-[11px]",
          {
            "cursor-not-allowed opacity-50": props.disabled,
            "cursor-pointer hover:bg-[#D4E2FF] hover:outline-none": !props.disabled,
          },
          className
        )}
      >
        {!hideIcon &&
          (props.icon ? (
            <Icon icon={props.icon} className="text-[#8D8D8D]" />
          ) : (
            <Icon icon="Documentation" className="opacity-0" />
          ))}
        <div className="flex w-full items-center gap-2.5">
          <span>{props.label}</span>

          {props.shortcut && <div className="ml-auto text-[#8D8D8D]">{props.shortcut.join("")}</div>}
        </div>
      </MenuPrimitive.Item>
    );
  }
);
