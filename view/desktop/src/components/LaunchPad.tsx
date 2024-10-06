import Accordion from "./DraggableAccordion";
import SidebarHeader from "./SidebarHeader";
import { useAppDispatch, RootState } from "@/store";
import { IAccordion, setAccordions, setPreferredSize } from "@/store/accordion/accordionSlice";
import { useSelector } from "react-redux";
import * as DesktopComponents from ".";
import { useRef, useEffect } from "react";
import { ImperativePanelGroupHandle, Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import { getPreferredSizeById } from "@/store/accordion/accordionSelectors";
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

// 1. Сколько составляет % от числа
const calculatePercentageOfNumber = (percentage: number, number: number): number => {
  const coefficient = number / 100;
  return coefficient * percentage;
};

// 2. Сколько процентов составляет число 1 от числа 2
const calculatePercentageOfPart = (part: number, total: number): number => {
  const coefficient = total / part;
  return 100 / coefficient;
};

// 3. Прибавить процент к числу
const addPercentageToNumber = (percentage: number, number: number): number => {
  const coefficient = number / 100;
  const percentageValue = coefficient * percentage;
  return number + percentageValue;
};

// 4. Вычесть процент из числа
const subtractPercentageFromNumber = (percentage: number, number: number): number => {
  const coefficient = number / 100;
  const percentageValue = coefficient * percentage;
  return number - percentageValue;
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
  const accordions = useSelector((state: RootState) => state.accordion.accordion);
  // const preferredSizes = useSelector((state: RootState) => state.accordion.preferredSizes);

  const panelsRef = useRef<ImperativePanelGroupHandle>(null);

  const minCollapsedSize = useRef<number | undefined>(5.18);
  const maxCollapsedSize = useRef<number | undefined>(5.18);
  const minPanelSize = useRef<number | undefined>(14.8);
  const maxPanelSize = useRef<number | undefined>(44.44);

  useEffect(() => {
    if (!panelsRef.current) return;

    const htmlDiv = document.querySelector(`[data-panel-group-id='${panelsRef.current.getId()}']`);
    if (!htmlDiv) return;

    minCollapsedSize.current = calculatePercentageOfPart(35, htmlDiv.clientHeight);
    maxCollapsedSize.current = calculatePercentageOfPart(35, htmlDiv.clientHeight);
    minPanelSize.current = calculatePercentageOfPart(100, htmlDiv.clientHeight);
    maxPanelSize.current = calculatePercentageOfPart(300, htmlDiv.clientHeight);
  }, [panelsRef]);

  const toggleAccordion = (index: number) => {
    const updatedAccordions = accordions.map((accordion, i) =>
      i === index ? { ...accordion, isOpen: !accordion.isOpen } : accordion
    );

    dispatch(setAccordions(updatedAccordions));
  };

  const handlePanelResize = (id: IAccordion["id"], size: number) => {
    dispatch(setPreferredSize({ id, size }));
  };

  return (
    <div className="h-[calc(100%_-_42px)] overflow-auto">
      <PanelGroup ref={panelsRef} direction="vertical" id={"sidebar"}>
        {accordions.map((accordion, index) => (
          <>
            <Panel
              key={accordion.id}
              id={String(accordion.id)}
              order={index}
              minSize={accordion.isOpen ? minPanelSize.current : minCollapsedSize.current}
              maxSize={accordion.isOpen ? maxPanelSize.current : maxCollapsedSize.current}
              onResize={(newSize) => {
                if (accordion.isOpen) handlePanelResize(accordion.id, newSize);
              }}
            >
              <Accordion {...accordion} index={index} handleClick={() => toggleAccordion(index)}>
                {getDesktopComponentByName(accordion.content as DesktopComponentsOmitted)}
              </Accordion>
            </Panel>
            <PanelResizeHandle className="h-px w-full bg-stone-100" key={`handle-${accordion.id}`} />
            {index === accordions.length - 1 && <Panel key={accordion.id + 1} order={index + 1}></Panel>}
          </>
        ))}
      </PanelGroup>
    </div>
  );
};
