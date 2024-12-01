import { cn } from "../utils/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";
import Icon from "../Icon";
import { ScopedProps } from "./types";

export type CheckboxItemElement = ElementRef<typeof MenuPrimitive.CheckboxItem>;
export type CheckboxItemProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.CheckboxItem>> & {
  label: string;
  shortcut?: string[];
  disabled?: boolean;
};

export const CheckboxItem = forwardRef<CheckboxItemElement, CheckboxItemProps>(
  (props: ScopedProps<CheckboxItemProps>, forwardedRef) => {
    return (
      <MenuPrimitive.CheckboxItem
        {...props}
        ref={forwardedRef}
        className={cn("flex items-center gap-1.5 rounded px-2 py-1", {
          "cursor-not-allowed opacity-50": props.disabled,
          "cursor-pointer hover:bg-[#D4E2FF] hover:outline-none": !props.disabled,
        })}
      >
        {props.checked ? <Icon icon="CheckIconGreen" /> : <Icon icon="CheckIconGreen" className="opacity-0" />}

        <div className="flex w-full items-center gap-2.5">
          <span>{props.label}</span>

          {props.shortcut && <div className="ml-auto text-[#8D8D8D]">{props.shortcut.join("")}</div>}
        </div>
      </MenuPrimitive.CheckboxItem>
    );
  }
);
