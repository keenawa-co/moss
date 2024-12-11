import { cloneElement, ComponentPropsWithoutRef, isValidElement, ReactElement } from "react";

import { UniqueIdentifier } from "@dnd-kit/core";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { cn } from "@repo/moss-ui";

interface DNDItemWrapperProps extends Omit<ComponentPropsWithoutRef<"div">, "id"> {
  id: UniqueIdentifier;
  idleClassName?: string;
  draggingClassName?: string;
}

export const TestDNDWrapper = ({ id, idleClassName, draggingClassName, children, ...props }: DNDItemWrapperProps) => {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id,
  });

  const style = {
    transform: CSS.Translate.toString(transform),
    transition,
    ...props.style,
  };

  if (!isValidElement(children)) {
    throw new Error("DNDSortableItemWrapper requires exactly one valid React element as a child.");
  }

  const child = children as ReactElement;

  return cloneElement(child, {
    ref: setNodeRef,
    ...attributes,
    ...listeners,
    style: { ...children.props.style, ...style },
    className: cn(children.props.className, isDragging ? draggingClassName : idleClassName, props.className),
  });
};
