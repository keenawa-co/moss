import * as React from "react";
import * as DropdownMenuPrimitive from "@radix-ui/react-dropdown-menu";
import { Circle } from "lucide-react";

import { cn } from "./lib/utils";

const DropdownMenu = DropdownMenuPrimitive.Root;

const DropdownMenuTrigger = DropdownMenuPrimitive.Trigger;

const DropdownMenuGroup = DropdownMenuPrimitive.Group;

const DropdownMenuSub = DropdownMenuPrimitive.Sub;

const DropdownMenuRadioGroup = DropdownMenuPrimitive.RadioGroup;

const DropdownMenuPortal = DropdownMenuPrimitive.Portal;

const DropdownMenuArrow = DropdownMenuPrimitive.Arrow;

// CONTENT

const DropdownMenuContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Content> & {
    withArrow?: boolean;
  }
>(({ withArrow, sideOffset = 4, className, ...props }, ref) => (
  <DropdownMenuPrimitive.Portal>
    <DropdownMenuPrimitive.Content
      ref={ref}
      sideOffset={sideOffset}
      className={cn(
        "z-50 min-w-[16rem] text-xs overflow-hidden rounded-lg bg-[#1E1E1E] py-3 px-3 text-zinc-100 shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        className
      )}
      {...props}
    >
      {props.children}
      {withArrow && <DropdownMenuArrow />}
    </DropdownMenuPrimitive.Content>
  </DropdownMenuPrimitive.Portal>
));
DropdownMenuContent.displayName = DropdownMenuPrimitive.Content.displayName;

const DropdownMenuItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Item>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Item>
>(({ className, ...props }, ref) => (
  <DropdownMenuPrimitive.Item
    ref={ref}
    className={cn(
      "relative flex select-none hover:bg-[#0a99ff] items-center rounded py-[3px] pl-8 pr-1 outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
      className
    )}
    {...props}
  />
));
DropdownMenuItem.displayName = DropdownMenuPrimitive.Item.displayName;

// SUB CONTENT

const DropdownMenuSubTrigger = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.SubTrigger>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.SubTrigger>
>(({ className, children, ...props }, ref) => (
  <DropdownMenuPrimitive.SubTrigger
    ref={ref}
    className={cn(
      "relative flex cursor-default select-none font-normal items-center hover:bg-[#0a99ff] rounded py-[3px] pl-8 outline-none focus:bg-accent data-[state=open]:bg-accent",
      className
    )}
    {...props}
  >
    {children}
    {/* TODO change inline svg to icon component */}
    <svg className="ml-auto" width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M5 4L6.71429 6L5 8" stroke="#FAFAFA" stroke-width="1.25" stroke-linecap="round" />
    </svg>
  </DropdownMenuPrimitive.SubTrigger>
));
DropdownMenuSubTrigger.displayName = DropdownMenuPrimitive.SubTrigger.displayName;

const DropdownMenuSubContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.SubContent>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.SubContent>
>(({ className, ...props }, ref) => (
  <DropdownMenuPrimitive.SubContent
    sideOffset={16 || props.sideOffset}
    ref={ref}
    className={cn(
      "z-50 min-w-[8rem] text-xs overflow-hidden rounded-lg bg-[#1E1E1E] -mt-3 py-3 px-3 text-zinc-100 shadow-lg data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
      className
    )}
    {...props}
  />
));
DropdownMenuSubContent.displayName = DropdownMenuPrimitive.SubContent.displayName;

// CHECKBOX

const DropdownMenuCheckboxItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.CheckboxItem>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.CheckboxItem> & {
    closeOnSelect?: boolean;
    onSelect?: ((event: Event) => void) | undefined;
  }
>(({ closeOnSelect = false, onSelect, className, children, checked, ...props }, ref) => (
  <DropdownMenuPrimitive.CheckboxItem
    ref={ref}
    className={cn(
      "relative flex select-none items-center hover:bg-[#0a99ff] rounded cursor-pointer py-[3px] pl-8  outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
      className
    )}
    checked={checked}
    onSelect={(e) => {
      console.log(1);
      if (!closeOnSelect) {
        console.log(2);
        e.preventDefault();
      }
      console.log(3);
      onSelect?.(e);
    }}
    {...props}
  >
    <span className="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
      <DropdownMenuPrimitive.ItemIndicator>
        {/* TODO change inline svg to icon component */}
        <svg width="18" height="18" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            fill-rule="evenodd"
            clip-rule="evenodd"
            d="M3.24215 5.4494C3.45303 5.23852 3.79494 5.23852 4.00582 5.4494L5.32104 6.76462L7.9939 4.09176C8.20479 3.88087 8.5467 3.88087 8.75758 4.09176C8.96846 4.30264 8.96846 4.64455 8.75758 4.85543L5.70288 7.91013C5.492 8.12102 5.15009 8.12102 4.9392 7.91013L3.24215 6.21308C3.03126 6.00219 3.03126 5.66028 3.24215 5.4494Z"
            fill="#FAFAFA"
          />
        </svg>
      </DropdownMenuPrimitive.ItemIndicator>
    </span>
    {children}
  </DropdownMenuPrimitive.CheckboxItem>
));
DropdownMenuCheckboxItem.displayName = DropdownMenuPrimitive.CheckboxItem.displayName;

// RADIO

const DropdownMenuRadioItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.RadioItem>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.RadioItem>
>(({ className, children, ...props }, ref) => (
  <DropdownMenuPrimitive.RadioItem
    ref={ref}
    className={cn(
      "relative flex cursor-pointer select-none items-center hover:bg-[#0a99ff] rounded py-[3px] pl-8 outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
      className
    )}
    {...props}
  >
    <span className="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
      <DropdownMenuPrimitive.DropdownMenuItemIndicator>
        <Circle className="h-2 w-2 fill-current" />
      </DropdownMenuPrimitive.DropdownMenuItemIndicator>
    </span>
    {children}
  </DropdownMenuPrimitive.RadioItem>
));
DropdownMenuRadioItem.displayName = DropdownMenuPrimitive.RadioItem.displayName;

// GENERAL

const DropdownMenuIconWrapper = ({ children }: { children: React.ReactNode }) => {
  return <div className="absolute left-1 size-4 *:size-4">{children}</div>;
};

const DropdownMenuLabel = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Label>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Label> & {
    inset?: boolean;
  }
>(({ className, ...props }, ref) => (
  <DropdownMenuPrimitive.Label ref={ref} className={cn("py-[3px] font-semibold", className)} {...props} />
));
DropdownMenuLabel.displayName = DropdownMenuPrimitive.Label.displayName;

const DropdownMenuSeparator = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Separator>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Separator> & { fullWidth?: boolean }
>(({ fullWidth, className }, ref) => (
  <DropdownMenuPrimitive.Separator
    ref={ref}
    className={cn(
      "my-2 h-px bg-[#383838]",
      {
        "ml-7": !fullWidth,
      },
      className
    )}
  />
));
DropdownMenuSeparator.displayName = DropdownMenuPrimitive.Separator.displayName;

// TODO add functionality for DropdownMenuShortcut
const DropdownMenuShortcut = ({ className, ...props }: React.HTMLAttributes<HTMLSpanElement>) => {
  return <span className={cn("ml-auto opacity-60", className)} {...props} />;
};
DropdownMenuShortcut.displayName = "DropdownMenuShortcut";

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
  DropdownMenuIconWrapper,
};
