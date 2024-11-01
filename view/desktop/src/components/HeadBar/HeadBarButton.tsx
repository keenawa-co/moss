import { cn, Icon, Icons } from "@repo/ui";
import { ComponentPropsWithoutRef } from "react";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
interface HeadBarButtonProps extends ComponentPropsWithoutRef<"button"> {
  icon: Icons;
  label?: string;
  iconClassName?: string;
  sortableId?: string | number;
}

export const HeadBarButton = ({ icon, label, sortableId = -1, ...props }: HeadBarButtonProps) => {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id: sortableId,
  });

  const style = {
    transform: CSS.Translate.toString(transform),
    transition,
  };

  return (
    <button
      ref={setNodeRef}
      {...attributes}
      {...listeners}
      style={style}
      {...props}
      className={cn(
        "group z-40 flex items-center gap-1.5 rounded-[3px] font-normal transition-colors hover:bg-[#C6C6C6]",
        {
          "z-50 box-border cursor-grabbing border border-dashed border-[#727272] bg-[#e6e6e6] opacity-50 shadow-2xl":
            isDragging == true,
        },
        props.className
      )}
    >
      <Icon icon={icon} className={cn("group-active:text-black size-[18px] text-[#525252]", props.iconClassName)} />
      {label && <span className="group-active:text-black text-ellipsis text-[#161616]">{label}</span>}
    </button>
  );
};
