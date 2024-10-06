import Accordion from "./DraggableAccordion";
import SidebarHeader from "./SidebarHeader";
import { useAppDispatch, RootState } from "@/store";
import { IAccordion, setAccordions, setPreferredSize, setPreferredSizes } from "@/store/accordion/accordionSlice";
import { useSelector } from "react-redux";
import * as DesktopComponents from ".";
import { useRef, useEffect } from "react";
import { ImperativePanelGroupHandle, Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import { getPreferredSizeById } from "@/store/accordion/accordionSelectors";
import { Resizable, ResizablePanel } from "./Resizable";
import { AllotmentHandle } from "allotment";
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
  const ref = useRef<AllotmentHandle>(null);
  const dispatch = useAppDispatch();
  const accordions = useSelector((state: RootState) => state.accordion.accordion);
  const preferredSizes = useSelector((state: RootState) => state.accordion.preferredSizes);
  // const preferredSizes = useRef<number[] | undefined>(undefined);

  const toggleAccordion = (index: number) => {
    const updatedAccordions = accordions.map((accordion, i) =>
      i === index ? { ...accordion, isOpen: !accordion.isOpen } : accordion
    );

    dispatch(setAccordions(updatedAccordions));
  };

  const handleOnDragEnd = (sizes: number[]) => {
    // preferredSizes.current = sizes;
    dispatch(setPreferredSizes(sizes));
  };

  useEffect(() => {
    if (!ref.current || preferredSizes.length === 0) return;
    ref.current.resize(preferredSizes);
  }, [accordions]);

  return (
    <div className="h-[calc(100%_-_42px)] overflow-auto">
      <Resizable ref={ref} vertical className="h-full" proportionalLayout={false} onDragEnd={handleOnDragEnd}>
        {accordions.map((accordion, index) => (
          <ResizablePanel
            key={accordion.id}
            minSize={accordion.isOpen ? 100 : 35}
            maxSize={accordion.isOpen ? 300 : 35}
          >
            <Accordion {...accordion} index={index} handleClick={() => toggleAccordion(index)}>
              {getDesktopComponentByName(accordion.content as DesktopComponentsOmitted)}
            </Accordion>
          </ResizablePanel>
        ))}
        <ResizablePanel>
          <span />
        </ResizablePanel>
      </Resizable>
    </div>
  );
};
