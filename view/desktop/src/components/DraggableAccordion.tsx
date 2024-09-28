import { Icon } from "@repo/ui";
import { cn } from "@/utils";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
interface DraggableAccordionProps {
  id: number;
  title: string;
  isOpen?: boolean;
  handleClick: () => void;
  children: React.ReactNode[] | React.ReactNode;
}

const Accordion = ({ id, title, isOpen = false, handleClick, children }: DraggableAccordionProps) => {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  };

  return (
    <div>
      <div
        ref={setNodeRef}
        style={style}
        {...attributes}
        {...listeners}
        onClick={handleClick}
        className="flex items-center px-2 py-[5px]"
      >
        <div className={cn(`flex size-5 cursor-pointer items-center justify-center`, { "rotate-90": isOpen })}>
          <Icon icon="ArrowRight" className="text-xs" />
        </div>
        <span className="font-bold">{title}</span>
      </div>

      <div className={isOpen ? "text-gray-500 pl-6 text-xs" : "visually-hidden"}>{children}</div>
    </div>
  );
};

export default Accordion;
