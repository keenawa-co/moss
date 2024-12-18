import { ComponentPropsWithoutRef } from "react";

import { UniqueIdentifier } from "@dnd-kit/core";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { cn } from "@repo/moss-ui";

interface DNDItemWrapperProps extends Omit<ComponentPropsWithoutRef<"div">, "id"> {
  id: UniqueIdentifier;
  idleClassName?: string;
  draggingClassName?: string;
}

export const DNDSortableItemWrapper = ({ id, idleClassName, draggingClassName, ...props }: DNDItemWrapperProps) => {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id,
  });

  const style = {
    transform: CSS.Translate.toString(transform),
    transition,
  };

  return (
    <div
      ref={setNodeRef}
      {...attributes}
      {...listeners}
      style={style}
      className={cn(isDragging ? draggingClassName : idleClassName, props.className)}
    >
      {props.children}
    </div>
  );
};
