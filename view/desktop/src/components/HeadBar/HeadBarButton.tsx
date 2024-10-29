import { Icon, Icons } from "@repo/ui";
import { ComponentPropsWithoutRef } from "react";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
interface HeadBarButtonProps extends ComponentPropsWithoutRef<"button"> {
  icon: Icons;
  label?: string;
  sortableId?: string | number; // Renamed to avoid conflict
}

export const HeadBarButton = ({ icon, label, sortableId = -1, ...props }: HeadBarButtonProps) => {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: sortableId });

  const style = {
    transform: CSS.Translate.toString(transform),
    transition,
  };

  return (
    <button
      ref={setNodeRef}
      style={style}
      {...attributes}
      {...listeners}
      className="group flex items-center gap-1.5 transition-colors "
      {...props}
    >
      <Icon icon={icon} className="text-[#525252] group-hover:text-[#0065FF] group-active:text-[#0747A6]" />
      {label && <span className="text-[#161616] group-hover:text-[#0065FF] group-active:text-[#0747A6]">{label}</span>}
    </button>
  );
};
