import * as React from "react";
import * as DropdownMenuPrimitive from "@radix-ui/react-dropdown-menu";

import { cn } from "./lib/utils";

import {
  DropdownMenuCheckboxItemProps,
  DropdownMenuContentProps,
  DropdownMenuGroupProps,
  DropdownMenuItemProps,
  DropdownMenuLabelProps,
  DropdownMenuProps,
  DropdownMenuRadioItemProps,
  DropdownMenuSeparatorProps,
  DropdownMenuSubTriggerProps,
  DropdownMenuTriggerProps,
  DropdownMenuSubProps,
  DropdownMenuRadioGroupProps,
  DropdownMenuPortalProps,
  DropdownMenuArrowProps,
  DropdownMenuSubContentProps,
} from "./DropdownMenu.types";

import Icon from "./Icon";

// GENERAL

const TextStyles = "text-xs font-light text-white";
const ItemStyles =
  "py-2 px-2 hover:bg-[#0a99ff] group rounded-lg relative flex gap-3.5 select-none items-center outline-none transition-colors data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
const ContentStyles =
  "z-50 overflow-hidden rounded-xl bg-[#1E1E1E] py-1.5 px-1.5 shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
const IconStyles = "text-white hover:text-white w-4 h-4";

const DropdownMenu: React.FC<DropdownMenuProps> = DropdownMenuPrimitive.Root;

const DropdownMenuTrigger: React.FC<DropdownMenuTriggerProps> = DropdownMenuPrimitive.Trigger;

const DropdownMenuGroup: React.FC<DropdownMenuGroupProps> = DropdownMenuPrimitive.Group;

const DropdownMenuPortal: React.FC<DropdownMenuPortalProps> = DropdownMenuPrimitive.Portal;

const DropdownMenuArrow: React.FC<DropdownMenuArrowProps> = DropdownMenuPrimitive.Arrow;

const DropdownMenuLabel = ({ children, className, ...props }: DropdownMenuLabelProps) => (
  <DropdownMenuPrimitive.Label className={cn(ItemStyles, TextStyles, "font-semibold", className)} {...props}>
    {children}
  </DropdownMenuPrimitive.Label>
);

const DropdownMenuSeparator = ({ className }: DropdownMenuSeparatorProps) => (
  <DropdownMenuPrimitive.Separator className={cn("h-px mx-0.5  my-2 bg-[#383838]", className)} />
);

// TODO add functionality for DropdownMenuShortcut
const DropdownMenuShortcut = ({ className, ...props }: React.HTMLAttributes<HTMLSpanElement>) => {
  return <span className={cn("ml-auto opacity-60", className)} {...props} />;
};

// CONTENT

const DropdownMenuContent = ({
  withArrow = false,
  sideOffset = 4,
  className,
  children,
  ...props
}: DropdownMenuContentProps) => (
  <DropdownMenuPrimitive.Portal>
    <DropdownMenuPrimitive.Content
      sideOffset={sideOffset}
      className={cn(ContentStyles, TextStyles, "min-w-64 ", className)}
      {...props}
    >
      {children}
      {withArrow && <DropdownMenuArrow />}
    </DropdownMenuPrimitive.Content>
  </DropdownMenuPrimitive.Portal>
);

const DropdownMenuItem = ({ icon, className, children, ...props }: DropdownMenuItemProps) => (
  <DropdownMenuPrimitive.Item className={cn(ItemStyles, className)} {...props}>
    {icon ? <Icon icon={icon} className={IconStyles} /> : <div className="size-4" />}

    <div className="flex w-full justify-between">{children}</div>
  </DropdownMenuPrimitive.Item>
);

// SUB CONTENT

const DropdownMenuSub: React.FC<DropdownMenuSubProps> = DropdownMenuPrimitive.Sub;

const DropdownMenuSubTrigger = ({ icon, className, children, ...props }: DropdownMenuSubTriggerProps) => (
  <DropdownMenuPrimitive.SubTrigger className={cn(ItemStyles, "pr-1 cursor-default", className)} {...props}>
    {icon ? <Icon icon={icon} className={IconStyles} /> : <div className="size-4" />}

    <div className="flex w-full justify-between">
      {children}
      <Icon icon="DropdownMenuSubTriggerArrow" className="size-3.5 pt-[2px]" />
    </div>
  </DropdownMenuPrimitive.SubTrigger>
);

const DropdownMenuSubContent = ({ className, children, ...props }: DropdownMenuSubContentProps) => (
  <DropdownMenuPrimitive.SubContent
    sideOffset={16 || props.sideOffset}
    className={cn(ContentStyles, TextStyles, "min-w-48 -mt-1.5", className)}
    {...props}
  >
    {children}
  </DropdownMenuPrimitive.SubContent>
);

// CHECKBOX

const DropdownMenuCheckboxItem = ({
  closeOnSelect = false,
  onSelect,
  className,
  children,
  checked,
  ...props
}: DropdownMenuCheckboxItemProps) => (
  <DropdownMenuPrimitive.CheckboxItem
    className={cn(ItemStyles, className)}
    checked={checked}
    onSelect={(e) => {
      if (!closeOnSelect) e.preventDefault();
      onSelect?.(e);
    }}
    {...props}
  >
    {checked ? <Icon icon="DropdownMenuCheckboxIndicator" className={IconStyles} /> : <div className="size-4" />}
    {children}
  </DropdownMenuPrimitive.CheckboxItem>
);

// RADIO

const DropdownMenuRadioGroup: React.FC<DropdownMenuRadioGroupProps> = DropdownMenuPrimitive.RadioGroup;

const DropdownMenuRadioItem = ({ className, children, ...props }: DropdownMenuRadioItemProps) => (
  <DropdownMenuPrimitive.RadioItem className={cn(ItemStyles, "cursor-pointer", className)} {...props}>
    <DropdownMenuPrimitive.DropdownMenuItemIndicator className="-mr-[22px]">
      <Icon icon="DropdownMenuRadioIndicator" className={cn(IconStyles, "size-2")} />
    </DropdownMenuPrimitive.DropdownMenuItemIndicator>

    <div className="pl-6">{children}</div>
  </DropdownMenuPrimitive.RadioItem>
);

export {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuCheckboxItem,
  DropdownMenuRadioItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuGroup,
  DropdownMenuPortal,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuRadioGroup,
  DropdownMenuArrow,
};
