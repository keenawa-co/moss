import { cn } from "../utils/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";
import { ScopedProps } from "./types";

export type ContentElement = ElementRef<typeof MenuPrimitive.Content>;
export type ContentProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.Content>>;

export const Content = forwardRef<ContentElement, ContentProps>((props, forwardedRef) => {
  return (
    <MenuPrimitive.Content
      {...props}
      className={cn("rounded-lg border border-[#c9ccd6] bg-white px-3 py-1.5 shadow-lg", props.className)}
      ref={forwardedRef}
    />
  );
});
