import { DragDropContext, Droppable, DroppableProvided, DropResult } from "@hello-pangea/dnd";
import Accordion from "./DraggableAccordion";
import { Resizable, ResizablePanel } from "./Resizable";
import SidebarHeader from "./SidebarHeader";
import { useAppDispatch, RootState } from "@/store";
import { setAccordion } from "@/store/accordion/accordionSlice";
import { useSelector } from "react-redux";
import * as DesktopComponents from ".";
import { swapByIndex } from "@/store/accordion/accordionHelpers";
import { useRef } from "react";
type DesktopComponentKeys = keyof typeof DesktopComponents;
type OmittedComponents = Omit<
  Record<DesktopComponentKeys, any>,
  "RootLayout" | "SidebarLayout" | "ContentLayout" | "PropertiesLayout"
>;
type DesktopComponentsOmitted = keyof OmittedComponents;

const getDesktopComponentByName = (name: DesktopComponentsOmitted) => {
  if (!DesktopComponents[name]) return <div>{name}</div>;

  const Tag = DesktopComponents[name];
  return <Tag />;
};

const LaunchPad = () => {
  const ResizableRef = useRef(null);

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

    dispatch(setAccordion(updatedAccordion));
  };

  const handleDragEnd = (result: DropResult) => {
    console.log(result);
    const oldIndex = result.source.index;
    const newIndex = result.destination?.index;

    if (newIndex == undefined || oldIndex === newIndex) return;

    const updated = swapByIndex(accordion, oldIndex, newIndex);
    console.log(updated);
    dispatch(setAccordion(updated));
  };

  return (
    <DragDropContext onDragEnd={handleDragEnd}>
      <Droppable droppableId="list">
        {(provided: DroppableProvided) => (
          <div ref={provided.innerRef} {...provided.droppableProps} className="h-full">
            <Resizable proportionalLayout={false} vertical minSize={100}>
              {accordion.map((accordion, index) => (
                <Accordion key={accordion.id} {...accordion} index={index} handleClick={() => toggleAccordion(index)}>
                  {getDesktopComponentByName(accordion.content as DesktopComponentsOmitted)}
                </Accordion>
              ))}
            </Resizable>
            {provided.placeholder}
          </div>
        )}
      </Droppable>
    </DragDropContext>
  );
};
