import React, {
  ComponentPropsWithoutRef,
  ElementRef,
  FC,
  forwardRef,
  PointerEventHandler,
  ReactNode,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";

import { composeEventHandlers } from "@radix-ui/primitive";
import { createContextScope } from "@radix-ui/react-context";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { createMenuScope } from "@radix-ui/react-menu";
import { Primitive } from "@radix-ui/react-primitive";
import { useCallbackRef } from "@radix-ui/react-use-callback-ref";
import { useControllableState } from "@radix-ui/react-use-controllable-state";

import * as CustomPrimitive from "../index";
import { ScopedProps } from "../types";

function whenTouchOrPen<E>(handler: PointerEventHandler<E>): PointerEventHandler<E> {
  return (event) => (event.pointerType !== "mouse" ? handler(event) : undefined);
}

type Direction = "ltr" | "rtl";
type Point = { x: number; y: number };

/* -------------------------------------------------------------------------------------------------
 * ContextMenu
 * -----------------------------------------------------------------------------------------------*/

const CONTEXT_MENU_NAME = "ContextMenu";

const [createContextMenuContext, createContextMenuScope] = createContextScope(CONTEXT_MENU_NAME, [createMenuScope]);
const useMenuScope = createMenuScope();

type ContextMenuContextValue = {
  open: boolean;
  onOpenChange(open: boolean): void;
  modal: boolean;
};

const [ContextMenuProvider, useContextMenuContext] =
  createContextMenuContext<ContextMenuContextValue>(CONTEXT_MENU_NAME);

interface ContextMenuProps {
  children?: ReactNode;
  onOpenChange?(open: boolean): void;
  dir?: Direction;
  modal?: boolean;
}

const ContextMenu: FC<ContextMenuProps> = (props: ScopedProps<ContextMenuProps>) => {
  const { __scopeContextMenu, children, onOpenChange, dir, modal = true } = props;
  const [open, setOpen] = useState(false);
  const menuScope = useMenuScope(__scopeContextMenu);
  const handleOpenChangeProp = useCallbackRef(onOpenChange);

  const handleOpenChange = useCallback(
    (open: boolean) => {
      setOpen(open);
      handleOpenChangeProp(open);
    },
    [handleOpenChangeProp]
  );

  return (
    <ContextMenuProvider scope={__scopeContextMenu} open={open} onOpenChange={handleOpenChange} modal={modal}>
      <MenuPrimitive.Root {...menuScope} dir={dir} open={open} onOpenChange={handleOpenChange} modal={modal}>
        {children}
      </MenuPrimitive.Root>
    </ContextMenuProvider>
  );
};

/* -------------------------------------------------------------------------------------------------
 * ContextMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

type ContextMenuTriggerElement = ElementRef<typeof Primitive.span>;
type PrimitiveSpanProps = ComponentPropsWithoutRef<typeof Primitive.span>;
interface ContextMenuTriggerProps extends PrimitiveSpanProps {
  disabled?: boolean;
}

const ContextMenuTrigger = forwardRef<ContextMenuTriggerElement, ContextMenuTriggerProps>(
  (props: ScopedProps<ContextMenuTriggerProps>, forwardedRef) => {
    const { __scopeContextMenu, disabled = false, ...triggerProps } = props;
    const context = useContextMenuContext("ContextMenuTrigger", __scopeContextMenu);
    const menuScope = useMenuScope(__scopeContextMenu);
    const pointRef = useRef<Point>({ x: 0, y: 0 });
    const virtualRef = useRef({
      getBoundingClientRect: () => DOMRect.fromRect({ width: 0, height: 0, ...pointRef.current }),
    });
    const longPressTimerRef = useRef(0);
    const clearLongPress = useCallback(() => window.clearTimeout(longPressTimerRef.current), []);
    const handleOpen = (event: React.MouseEvent<HTMLSpanElement> | React.PointerEvent<HTMLSpanElement>) => {
      pointRef.current = { x: event.clientX, y: event.clientY };
      context.onOpenChange(true);
    };

    useEffect(() => clearLongPress, [clearLongPress]);
    useEffect(() => void (disabled && clearLongPress()), [disabled, clearLongPress]);

    return (
      <>
        <MenuPrimitive.Anchor {...menuScope} virtualRef={virtualRef} />
        <Primitive.span
          data-state={context.open ? "open" : "closed"}
          data-disabled={disabled ? "" : undefined}
          {...triggerProps}
          ref={forwardedRef}
          // prevent iOS context menu from appearing
          style={{ WebkitTouchCallout: "none", ...props.style }}
          // if trigger is disabled, enable the native Context Menu
          onContextMenu={
            disabled
              ? props.onContextMenu
              : composeEventHandlers(props.onContextMenu, (event) => {
                  // clearing the long press here because some platforms already support
                  // long press to trigger a `contextmenu` event
                  clearLongPress();
                  handleOpen(event);
                  event.preventDefault();
                })
          }
          onPointerDown={
            disabled
              ? props.onPointerDown
              : composeEventHandlers(
                  props.onPointerDown,
                  whenTouchOrPen((event) => {
                    // clear the long press here in case there's multiple touch points
                    clearLongPress();
                    longPressTimerRef.current = window.setTimeout(() => handleOpen(event), 700);
                  })
                )
          }
          onPointerMove={
            disabled ? props.onPointerMove : composeEventHandlers(props.onPointerMove, whenTouchOrPen(clearLongPress))
          }
          onPointerCancel={
            disabled
              ? props.onPointerCancel
              : composeEventHandlers(props.onPointerCancel, whenTouchOrPen(clearLongPress))
          }
          onPointerUp={
            disabled ? props.onPointerUp : composeEventHandlers(props.onPointerUp, whenTouchOrPen(clearLongPress))
          }
        />
      </>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * ContextMenuPortal
 * -----------------------------------------------------------------------------------------------*/

type MenuPortalProps = ComponentPropsWithoutRef<typeof MenuPrimitive.Portal>;
// eslint-disable-next-line
interface ContextMenuPortalProps extends MenuPortalProps {}

const ContextMenuPortal: FC<ContextMenuPortalProps> = (props: ScopedProps<ContextMenuPortalProps>) => {
  const { __scopeContextMenu, ...portalProps } = props;
  const menuScope = useMenuScope(__scopeContextMenu);
  return <MenuPrimitive.Portal {...menuScope} {...portalProps} />;
};

/* -------------------------------------------------------------------------------------------------
 * ContextMenuContent
 * -----------------------------------------------------------------------------------------------*/

const ContextMenuContent = forwardRef<CustomPrimitive.ContentElement, CustomPrimitive.ContentProps>(
  (props, forwardedRef) => {
    const { __scopeContextMenu, ...contentProps } = props;
    const context = useContextMenuContext("ContextMenuContent", __scopeContextMenu);
    const menuScope = useMenuScope(__scopeContextMenu);
    const hasInteractedOutsideRef = useRef(false);

    return (
      <CustomPrimitive.Content
        {...menuScope}
        {...contentProps}
        ref={forwardedRef}
        side="right"
        sideOffset={2}
        align="start"
        onCloseAutoFocus={(event) => {
          props.onCloseAutoFocus?.(event);

          if (!event.defaultPrevented && hasInteractedOutsideRef.current) {
            event.preventDefault();
          }

          hasInteractedOutsideRef.current = false;
        }}
        onInteractOutside={(event) => {
          props.onInteractOutside?.(event);

          if (!event.defaultPrevented && !context.modal) hasInteractedOutsideRef.current = true;
        }}
        style={{
          ...props.style,
        }}
      />
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * ContextMenuItem
 * -----------------------------------------------------------------------------------------------*/

const ContextMenuItem = forwardRef<CustomPrimitive.ItemElement, CustomPrimitive.ItemProps>((props, forwardedRef) => {
  const { __scopeContextMenu, ...itemProps } = props;
  const menuScope = useMenuScope(__scopeContextMenu);
  return <CustomPrimitive.Item {...menuScope} {...itemProps} ref={forwardedRef} />;
});

/* -------------------------------------------------------------------------------------------------
 * ContextMenuCheckboxItem
 * -----------------------------------------------------------------------------------------------*/

const ContextMenuCheckboxItem = forwardRef<CustomPrimitive.CheckboxItemElement, CustomPrimitive.CheckboxItemProps>(
  (props, forwardedRef) => {
    const { __scopeContextMenu, ...checkboxItemProps } = props;
    const menuScope = useMenuScope(__scopeContextMenu);
    return <CustomPrimitive.CheckboxItem {...menuScope} {...checkboxItemProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * ContextMenuRadioGroup
 * -----------------------------------------------------------------------------------------------*/

const ContextMenuRadioGroup = forwardRef<CustomPrimitive.RadioGroupElement, CustomPrimitive.RadioGroupProps>(
  (props, forwardedRef) => {
    const { __scopeContextMenu, ...radioGroupProps } = props;
    const menuScope = useMenuScope(__scopeContextMenu);
    return <CustomPrimitive.RadioGroup {...menuScope} {...radioGroupProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * ContextMenuRadioItem
 * -----------------------------------------------------------------------------------------------*/

const ContextMenuRadioItem = forwardRef<CustomPrimitive.RadioItemElement, CustomPrimitive.RadioItemProps>(
  (props, forwardedRef) => {
    const { __scopeContextMenu, ...radioGroupProps } = props;
    const menuScope = useMenuScope(__scopeContextMenu);
    return <CustomPrimitive.RadioItem {...menuScope} {...radioGroupProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * ContextMenuSub
 * -----------------------------------------------------------------------------------------------*/

interface ContextMenuSubProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
}

const ContextMenuSub: React.FC<ContextMenuSubProps> = (props: ScopedProps<ContextMenuSubProps>) => {
  const { __scopeContextMenu, children, onOpenChange, open: openProp, defaultOpen } = props;
  const menuScope = useMenuScope(__scopeContextMenu);
  const [open, setOpen] = useControllableState({
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
 * ContextMenuSubTrigger
 * -----------------------------------------------------------------------------------------------*/

const ContextMenuSubTrigger = forwardRef<CustomPrimitive.SubTriggerElement, CustomPrimitive.SubTriggerProps>(
  (props, forwardedRef) => {
    const { __scopeContextMenu, ...triggerItemProps } = props;
    const menuScope = useMenuScope(__scopeContextMenu);
    return <CustomPrimitive.SubTrigger {...menuScope} {...triggerItemProps} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * ContextMenuSubContent
 * -----------------------------------------------------------------------------------------------*/

const ContextMenuSubContent = forwardRef<CustomPrimitive.SubContentElement, CustomPrimitive.SubContentProps>(
  (props, forwardedRef) => {
    const { __scopeContextMenu, ...subContentProps } = props;
    const menuScope = useMenuScope(__scopeContextMenu);

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

const Root = ContextMenu;
const Trigger = ContextMenuTrigger;
const Portal = ContextMenuPortal;
const Content = ContextMenuContent;
const Item = ContextMenuItem;
const Separator = CustomPrimitive.Separator;
const CheckboxItem = ContextMenuCheckboxItem;
const RadioGroup = ContextMenuRadioGroup;
const RadioItem = ContextMenuRadioItem;
const Sub = ContextMenuSub;
const SubTrigger = ContextMenuSubTrigger;
const SubContent = ContextMenuSubContent;

export {
  CheckboxItem,
  Content,
  createContextMenuScope,
  RadioGroup,
  RadioItem,
  Portal,
  Root,
  Separator,
  Item,
  Sub,
  SubContent,
  SubTrigger,
  Trigger,
};

export type { ContextMenuPortalProps, ContextMenuProps, ContextMenuTriggerProps };
