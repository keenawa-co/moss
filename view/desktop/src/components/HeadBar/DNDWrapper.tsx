import { ComponentPropsWithoutRef } from "react";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { cn } from "@repo/ui";

interface DNDWrapperProps extends ComponentPropsWithoutRef<"span"> {
  sortableId: string | number;
  idleClassName?: string;
  draggingClassName?: string;
  children: React.ReactNode;
}

export const DNDWrapper = ({ sortableId, idleClassName, draggingClassName, ...props }: DNDWrapperProps) => {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id: sortableId,
  });

  const style = {
    transform: CSS.Translate.toString(transform),
    transition,
  };

  return (
    <span
      ref={setNodeRef}
      {...attributes}
      {...listeners}
      style={style}
      className={cn(isDragging ? draggingClassName : idleClassName, props.className)}
    >
      {props.children}
    </span>
  );
};
