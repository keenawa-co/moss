import { Icon } from "@repo/ui";
import { cn } from "@/utils";
import { Draggable, DraggableProvided, DraggableStateSnapshot } from "@hello-pangea/dnd";
import AccordionResizableBox from "./AccordionResizableBox";

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
    <Draggable draggableId={id.toString()} index={index}>
      {(provided: DraggableProvided, snapshot: DraggableStateSnapshot) => (
        <AccordionResizableBox accordionId={id} ref={provided.innerRef} {...provided.draggableProps} isOpen={isOpen}>
          <div className="DraggableAccordion h-full">
            <div onClick={handleClick} {...provided.dragHandleProps} className="flex items-center px-2 py-[5px]">
              <div className={cn(`flex size-5 cursor-pointer items-center justify-center`, { "rotate-90": isOpen })}>
                <Icon icon="ArrowRight" className="text-xs" />
              </div>
              <span className="font-bold">{title}</span>
            </div>

            <div className={isOpen ? "text-gray-500 h-full overflow-auto pl-6 text-xs" : "visually-hidden"}>
              {children}
            </div>
          </div>
        </AccordionResizableBox>
      )}
    </Draggable>
  );
};

export default Accordion;
