import { Icon } from "@repo/ui";
import { cn } from "@/utils";

interface DraggableAccordionProps {
  title: string;
  isOpen?: boolean;
  handleClick: () => void;
  children: React.ReactNode[] | React.ReactNode;
}

const Accordion = ({ title, isOpen = false, handleClick, children }: DraggableAccordionProps) => {
  return (
    <div>
      <div onClick={handleClick} className="flex items-center px-2 py-[5px]">
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
