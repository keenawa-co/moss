import { Icon } from "@repo/ui";
import { cn } from "@/utils";
import { Draggable, DraggableProvided, DraggableStateSnapshot } from "@hello-pangea/dnd";
import { ResizablePanel } from "./Resizable";
interface DraggableAccordionProps {
  id: number;
  title: string;
  isOpen?: boolean;
  index: number;
  handleClick: () => void;
  children: React.ReactNode[] | React.ReactNode;
}

const Accordion = ({ id, title, isOpen = false, index, handleClick, children }: DraggableAccordionProps) => {
  return (
    <Draggable draggableId={id.toString()} index={index}>
      {(provided: DraggableProvided, snapshot: DraggableStateSnapshot) => (
        <ResizablePanel className="h-full overflow-hidden" key={index} ref={provided.innerRef}>
          <div className="DraggableAccordion h-full">
            <div
              onClick={handleClick}
              {...provided.dragHandleProps}
              {...provided.draggableProps}
              className="flex items-center px-2 py-[5px]"
            >
              <div className={cn(`flex size-5 cursor-pointer items-center justify-center`, { "rotate-90": isOpen })}>
                <Icon icon="ArrowRight" className="text-xs" />
              </div>
              <span className="font-bold">
                {title} id: {id} - i: {index} - isOpen: {isOpen.toString()}
              </span>
            </div>

            <div
              className={
                isOpen && !snapshot.isDragging ? "text-gray-500 h-full overflow-auto pl-6 text-xs" : "visually-hidden"
              }
            >
              {children}
            </div>
          </div>
        </ResizablePanel>
      )}
    </Draggable>
  );
};

export default Accordion;
