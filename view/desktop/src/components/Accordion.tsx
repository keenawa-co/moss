import { Icon } from "../../../../packages/moss-ui/src";
import { cn } from "@/utils";

interface DraggableAccordionProps {
  id: number;
  title: string;
  isOpen?: boolean;
  index: number;
  handleClick: () => void;
  children: React.ReactNode[] | React.ReactNode;
}

const Accordion = ({ id, title, isOpen = false, index, handleClick, children, ...props }: DraggableAccordionProps) => {
  return (
    <div className={cn(`h-full`)}>
      <div onClick={handleClick} className="flex cursor-pointer items-center px-2 py-[5px]">
        <div className={cn(`flex size-5 items-center justify-center`, { "rotate-90": isOpen })}>
          <Icon icon="ArrowheadRight" className="text-xs" />
        </div>
        <span className="font-bold">{title}</span>
      </div>

      <div className={isOpen ? "text-gray-500 h-full overflow-auto pl-6 text-xs" : "visually-hidden"}>{children}</div>
    </div>
  );
};

export default Accordion;
