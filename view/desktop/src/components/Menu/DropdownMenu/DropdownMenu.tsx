import React, { forwardRef } from "react";

import { composeEventHandlers } from "@radix-ui/primitive";
import { composeRefs } from "@radix-ui/react-compose-refs";
import { createContextScope } from "@radix-ui/react-context";
import type { Scope } from "@radix-ui/react-context";
import { useId } from "@radix-ui/react-id";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { createMenuScope } from "@radix-ui/react-menu";
import { Primitive } from "@radix-ui/react-primitive";
import { useControllableState } from "@radix-ui/react-use-controllable-state";

import * as CustomPrimitive from "../index";

type Direction = "ltr" | "rtl";

/* -------------------------------------------------------------------------------------------------
 * DropdownMenu
 * -----------------------------------------------------------------------------------------------*/

const DROPDOWN_MENU_NAME = "DropdownMenu";

type ScopedProps<P> = P & { __scopeDropdownMenu?: Scope };
const [createDropdownMenuContext, createDropdownMenuScope] = createContextScope(DROPDOWN_MENU_NAME, [createMenuScope]);
const useMenuScope = createMenuScope();

type DropdownMenuContextValue = {
  triggerId: string;
  triggerRef: React.RefObject<HTMLButtonElement>;
  contentId: string;
  open: boolean;
  onOpenChange(open: boolean): void;
  onOpenToggle(): void;
  modal: boolean;
};

const [DropdownMenuProvider, useDropdownMenuContext] =
  createDropdownMenuContext<DropdownMenuContextValue>(DROPDOWN_MENU_NAME);

interface DropdownMenuProps {
  children?: React.ReactNode;
  dir?: Direction;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
  modal?: boolean;
}

const DropdownMenu: React.FC<DropdownMenuProps> = (props: ScopedProps<DropdownMenuProps>) => {
  const { __scopeDropdownMenu, children, dir, open: openProp, defaultOpen, onOpenChange, modal = true } = props;
  const menuScope = useMenuScope(__scopeDropdownMenu);
  const triggerRef = React.useRef<HTMLButtonElement>(null);
  const [open = false, setOpen] = useControllableState({
    prop: openProp,
    defaultProp: defaultOpen,
    onChange: onOpenChange,
  });

  return (
    <DropdownMenuProvider
      scope={__scopeDropdownMenu}
      triggerId={useId()}
      triggerRef={triggerRef}
      contentId={useId()}
      open={open}
      onOpenChange={setOpen}
      onOpenToggle={React.useCallback(() => setOpen((prevOpen) => !prevOpen), [setOpen])}
      modal={modal}
    >
      <MenuPrimitive.Root {...menuScope} open={open} onOpenChange={setOpen} dir={dir} modal={modal}>
        {children}
      </MenuPrimitive.Root>
    </DropdownMenuProvider>
  );
};

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

type DropdownMenuTriggerElement = React.ElementRef<typeof Primitive.button>;
type PrimitiveButtonProps = React.ComponentPropsWithoutRef<typeof Primitive.button>;
// eslint-disable-next-line
interface DropdownMenuTriggerProps extends PrimitiveButtonProps {}

const DropdownMenuTrigger = React.forwardRef<DropdownMenuTriggerElement, DropdownMenuTriggerProps>(
  (props: ScopedProps<DropdownMenuTriggerProps>, forwardedRef) => {
    const { __scopeDropdownMenu, disabled = false, ...triggerProps } = props;
    const context = useDropdownMenuContext("DropdownMenuTrigger", __scopeDropdownMenu);
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return (
      <MenuPrimitive.Anchor asChild {...menuScope}>
        <Primitive.button
          type="button"
          id={context.triggerId}
          aria-haspopup="menu"
          aria-expanded={context.open}
          aria-controls={context.open ? context.contentId : undefined}
          data-state={context.open ? "open" : "closed"}
          data-disabled={disabled ? "" : undefined}
          disabled={disabled}
          {...triggerProps}
          ref={composeRefs(forwardedRef, context.triggerRef)}
          onPointerDown={composeEventHandlers(props.onPointerDown, (event) => {
            // only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
            // but not when the control key is pressed (avoiding MacOS right click)
            if (!disabled && event.button === 0 && event.ctrlKey === false) {
              context.onOpenToggle();
              // prevent trigger focusing when opening
              // this allows the content to be given focus without competition
              // if (!context.open) event.preventDefault();
            } else {
              event.preventDefault();
            }
          })}
          onKeyDown={composeEventHandlers(props.onKeyDown, (event) => {
            if (disabled) return;
            if (["Enter", " "].includes(event.key)) context.onOpenToggle();
            if (event.key === "ArrowDown") context.onOpenChange(true);
            // prevent keydown from scrolling window / first focused item to execute
            // that keydown (inadvertently closing the menu)
            if (["Enter", " ", "ArrowDown"].includes(event.key)) event.preventDefault();
          })}
        />
      </MenuPrimitive.Anchor>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuPortal
 * -----------------------------------------------------------------------------------------------*/

type MenuPortalProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Portal>;
// eslint-disable-next-line
interface DropdownMenuPortalProps extends MenuPortalProps {}

const DropdownMenuPortal: React.FC<DropdownMenuPortalProps> = (props: ScopedProps<DropdownMenuPortalProps>) => {
  const { __scopeDropdownMenu, ...portalProps } = props;
  const menuScope = useMenuScope(__scopeDropdownMenu);
  return <MenuPrimitive.Portal {...menuScope} {...portalProps} />;
};

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuContent
 * -----------------------------------------------------------------------------------------------*/

type DropdownMenuContentElement = React.ElementRef<typeof MenuPrimitive.Content>;
type MenuContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Content>;
// eslint-disable-next-line
interface DropdownMenuContentProps extends Omit<MenuContentProps, "onEntryFocus"> {}

const DropdownMenuContent = React.forwardRef<DropdownMenuContentElement, DropdownMenuContentProps>(
  (props: ScopedProps<DropdownMenuContentProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...contentProps } = props;
    const context = useDropdownMenuContext("DropdownMenuContent", __scopeDropdownMenu);
    const menuScope = useMenuScope(__scopeDropdownMenu);
    const hasInteractedOutsideRef = React.useRef(false);

    return (
      <CustomPrimitive.Content
        id={context.contentId}
        aria-labelledby={context.triggerId}
        {...menuScope}
        {...contentProps}
        ref={forwardedRef}
        onCloseAutoFocus={composeEventHandlers(props.onCloseAutoFocus, (event) => {
          if (!hasInteractedOutsideRef.current) context.triggerRef.current?.focus();
          hasInteractedOutsideRef.current = false;
          // Always prevent auto focus because we either focus manually or want user agent focus
          event.preventDefault();
        })}
        onInteractOutside={composeEventHandlers(props.onInteractOutside, (event) => {
          const originalEvent = event.detail.originalEvent as PointerEvent;
          const ctrlLeftClick = originalEvent.button === 0 && originalEvent.ctrlKey === true;
          const isRightClick = originalEvent.button === 2 || ctrlLeftClick;
          if (!context.modal || isRightClick) hasInteractedOutsideRef.current = true;
        })}
        style={{
          ...props.style,
        }}
      />
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuGroup
 * -----------------------------------------------------------------------------------------------*/

type DropdownMenuGroupElement = React.ElementRef<typeof MenuPrimitive.Group>;
type MenuGroupProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Group>;
// eslint-disable-next-line
interface DropdownMenuGroupProps extends MenuGroupProps {}

const DropdownMenuGroup = React.forwardRef<DropdownMenuGroupElement, DropdownMenuGroupProps>(
  (props: ScopedProps<DropdownMenuGroupProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...groupProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <MenuPrimitive.Group {...menuScope} {...groupProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuLabel
 * -----------------------------------------------------------------------------------------------*/

type DropdownMenuLabelElement = React.ElementRef<typeof MenuPrimitive.Label>;
type MenuLabelProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Label>;
// eslint-disable-next-line
interface DropdownMenuLabelProps extends MenuLabelProps {}

const DropdownMenuLabel = React.forwardRef<DropdownMenuLabelElement, DropdownMenuLabelProps>(
  (props: ScopedProps<DropdownMenuLabelProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...labelProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <MenuPrimitive.Label {...menuScope} {...labelProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuItem
 * -----------------------------------------------------------------------------------------------*/

const DropdownMenuItem = React.forwardRef<CustomPrimitive.ItemElement, CustomPrimitive.ItemProps>(
  (props, forwardedRef) => {
    const { __scopeDropdownMenu, ...itemProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <CustomPrimitive.Item {...menuScope} {...itemProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuCheckboxItem
 * -----------------------------------------------------------------------------------------------*/

const DropdownMenuCheckboxItem = forwardRef<CustomPrimitive.CheckboxItemElement, CustomPrimitive.CheckboxItemProps>(
  (props: ScopedProps<CustomPrimitive.ItemProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...checkboxItemProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <CustomPrimitive.CheckboxItem {...menuScope} {...checkboxItemProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuRadioGroup
 * -----------------------------------------------------------------------------------------------*/

const DropdownMenuRadioGroup = forwardRef<CustomPrimitive.RadioGroupElement, CustomPrimitive.RadioGroupProps>(
  (props: ScopedProps<CustomPrimitive.RadioGroupProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...radioGroupProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <CustomPrimitive.RadioGroup {...menuScope} {...radioGroupProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuRadioItem
 * -----------------------------------------------------------------------------------------------*/

const DropdownMenuRadioItem = forwardRef<CustomPrimitive.RadioItemElement, CustomPrimitive.RadioItemProps>(
  (props: ScopedProps<CustomPrimitive.RadioItemProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...radioItemProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <CustomPrimitive.RadioItem {...menuScope} {...radioItemProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuItemIndicator
 * -----------------------------------------------------------------------------------------------*/

type DropdownMenuItemIndicatorElement = React.ElementRef<typeof MenuPrimitive.ItemIndicator>;
type MenuItemIndicatorProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.ItemIndicator>;
// eslint-disable-next-line
interface DropdownMenuItemIndicatorProps extends MenuItemIndicatorProps {}

const DropdownMenuItemIndicator = React.forwardRef<DropdownMenuItemIndicatorElement, DropdownMenuItemIndicatorProps>(
  (props: ScopedProps<DropdownMenuItemIndicatorProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...itemIndicatorProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <MenuPrimitive.ItemIndicator {...menuScope} {...itemIndicatorProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuArrow
 * -----------------------------------------------------------------------------------------------*/

type DropdownMenuArrowElement = React.ElementRef<typeof MenuPrimitive.Arrow>;
type MenuArrowProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Arrow>;
// eslint-disable-next-line
interface DropdownMenuArrowProps extends MenuArrowProps {}

const DropdownMenuArrow = React.forwardRef<DropdownMenuArrowElement, DropdownMenuArrowProps>(
  (props: ScopedProps<DropdownMenuArrowProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...arrowProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <MenuPrimitive.Arrow {...menuScope} {...arrowProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuSub
 * -----------------------------------------------------------------------------------------------*/

interface DropdownMenuSubProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
}

const DropdownMenuSub: React.FC<DropdownMenuSubProps> = (props: ScopedProps<DropdownMenuSubProps>) => {
  const { __scopeDropdownMenu, children, open: openProp, onOpenChange, defaultOpen } = props;
  const menuScope = useMenuScope(__scopeDropdownMenu);
  const [open = false, setOpen] = useControllableState({
    prop: openProp,
    defaultProp: defaultOpen,
    onChange: onOpenChange,
  });

  return (
    <MenuPrimitive.Sub {...menuScope} open={open} onOpenChange={setOpen}>
      {children}
    </MenuPrimitive.Sub>
  );
};

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuSubTrigger
 * -----------------------------------------------------------------------------------------------*/

const DropdownMenuSubTrigger = React.forwardRef<CustomPrimitive.SubTriggerElement, CustomPrimitive.SubTriggerProps>(
  (props: ScopedProps<CustomPrimitive.SubTriggerProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...subTriggerProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);
    return <CustomPrimitive.SubTrigger {...menuScope} {...subTriggerProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuSubContent
 * -----------------------------------------------------------------------------------------------*/

type DropdownMenuSubContentElement = React.ElementRef<typeof MenuPrimitive.Content>;
type MenuSubContentProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.SubContent>;
// eslint-disable-next-line
interface DropdownMenuSubContentProps extends MenuSubContentProps {}

const DropdownMenuSubContent = React.forwardRef<DropdownMenuSubContentElement, DropdownMenuSubContentProps>(
  (props: ScopedProps<DropdownMenuSubContentProps>, forwardedRef) => {
    const { __scopeDropdownMenu, ...subContentProps } = props;
    const menuScope = useMenuScope(__scopeDropdownMenu);

    return (
      <CustomPrimitive.SubContent
        {...menuScope}
        {...subContentProps}
        ref={forwardedRef}
        style={{
          ...props.style,
        }}
      />
    );
  }
);

/* -----------------------------------------------------------------------------------------------*/

const Root = DropdownMenu;
const Trigger = DropdownMenuTrigger;
const Portal = DropdownMenuPortal;
const Content = DropdownMenuContent;
const Group = DropdownMenuGroup;
const Label = DropdownMenuLabel;
const Item = DropdownMenuItem;
const CheckboxItem = DropdownMenuCheckboxItem;
const RadioGroup = DropdownMenuRadioGroup;
const RadioItem = DropdownMenuRadioItem;
const ItemIndicator = DropdownMenuItemIndicator;
const Separator = CustomPrimitive.Separator;
const Arrow = DropdownMenuArrow;
const Sub = DropdownMenuSub;
const SubTrigger = DropdownMenuSubTrigger;
const SubContent = DropdownMenuSubContent;

export {
  createDropdownMenuScope,
  Root,
  Trigger,
  Portal,
  Content,
  Group,
  Label,
  Item,
  CheckboxItem,
  RadioGroup,
  RadioItem,
  ItemIndicator,
  Separator,
  Arrow,
  Sub,
  SubTrigger,
  SubContent,
};
