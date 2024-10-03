import { DragDropContext, Droppable, DroppableProvided, DropResult } from "@hello-pangea/dnd";
import Accordion from "./DraggableAccordion";
import SidebarHeader from "./SidebarHeader";
import { useAppDispatch, RootState } from "@/store";
import { setAccordions } from "@/store/accordion/accordionSlice";
import { useSelector } from "react-redux";
import * as DesktopComponents from ".";
type DesktopComponentKeys = keyof typeof DesktopComponents;
type OmittedComponents = Omit<
  Record<DesktopComponentKeys, any>,
  "RootLayout" | "SidebarLayout" | "ContentLayout" | "PropertiesLayout"
>;
type DesktopComponentsOmitted = keyof OmittedComponents;

const getDesktopComponentByName = (name: DesktopComponentsOmitted) => {
  if (!DesktopComponents[name]) return <div>{name}</div>;

  const Tag = DesktopComponents[name];
  // FIXME: Type 'typeof Tag' is not assignable to type 'ElementType<any>'.
  //@ts-ignore
  return <Tag />;
};

const LaunchPad = () => {
  return (
    <>
      <SidebarHeader title="launchpad" />
      <DroppableContainer />
    </>
  );
};

export default LaunchPad;

const DroppableContainer = () => {
  const dispatch = useAppDispatch();
  const accordion = useSelector((state: RootState) => state.accordion.accordion);

  const toggleAccordion = (index: number) => {
    const updatedAccordion = accordion.map((accordion, i) =>
      i === index ? { ...accordion, isOpen: !accordion.isOpen } : accordion
    );

    dispatch(setAccordions(updatedAccordion));
  };

  const handleDragEnd = (result: DropResult) => {
    const { destination, source, draggableId } = result;

    if (!destination) return;
    if (destination.droppableId === source.droppableId && destination.index === source.index) return;

    const updatedAccordion = Array.from(accordion);
    const draggedAccordion = updatedAccordion.find((accordion) => accordion.id === Number(draggableId));

    if (!draggedAccordion) return;

    updatedAccordion.splice(source.index, 1);
    updatedAccordion.splice(destination.index, 0, draggedAccordion);

    dispatch(setAccordions(updatedAccordion));
  };

  return (
    <DragDropContext onDragEnd={handleDragEnd}>
      <Droppable droppableId="list">
        {(provided: DroppableProvided) => (
          <div ref={provided.innerRef} {...provided.droppableProps} className="h-full">
            <div className="h-[calc(100%_-_42px)] overflow-auto">
              {accordion.map((accordion, index) => (
                <Accordion key={accordion.id} {...accordion} index={index} handleClick={() => toggleAccordion(index)}>
                  {getDesktopComponentByName(accordion.content as DesktopComponentsOmitted)}
                </Accordion>
              ))}
              {provided.placeholder}
            </div>
          </div>
        )}
      </Droppable>
    </DragDropContext>
  );
};
