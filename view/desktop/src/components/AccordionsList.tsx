import { AllotmentHandle } from "allotment";
import { use } from "i18next";
import { useEffect, useRef } from "react";
import { useSelector } from "react-redux";

import { RootState, useAppDispatch } from "@/store";
import { setAccordions, setPreferredSizes } from "@/store/accordion/accordionSlice";

import * as DesktopComponents from ".";
import Accordion from "./Accordion";
import { Resizable, ResizablePanel } from "./Resizable";

type DesktopComponentKeys = keyof typeof DesktopComponents;
type OmittedComponents = Omit<
  Record<DesktopComponentKeys, unknown>,
  "RootLayout" | "ContentLayout" | "PropertiesLayout" | "Button" | "SidebarHeader"
>;
type DesktopComponentsOmitted = keyof OmittedComponents;
const getDesktopComponentByName = (name: DesktopComponentsOmitted) => {
  if (!DesktopComponents[name]) return <div>{name}</div>;

  const Tag = DesktopComponents[name];
  return <Tag />;
};

export const AccordionsList = () => {
  const ref = useRef<AllotmentHandle>(null);

  const dispatch = useAppDispatch();
  const accordions = useSelector((state: RootState) => state.accordion.accordion);
  const preferredSizes = useSelector((state: RootState) => state.accordion.preferredSizes);

  useEffect(() => {
    if (!ref.current || preferredSizes.length === 0 || accordions.length === 0) return;

    if (preferredSizes.length !== accordions.length) {
      console.warn("Preferred sizes and panels mismatch. Skipping resize.");
      return;
    }

    try {
      ref.current.resize(preferredSizes);
    } catch (error) {
      console.error("Error resizing panels:", error);
    }
  }, [accordions, preferredSizes, ref]);

  const toggleAccordion = (index: number) => {
    const updatedAccordions = accordions.map((accordion, i) =>
      i === index ? { ...accordion, isOpen: !accordion.isOpen } : accordion
    );

    dispatch(setAccordions(updatedAccordions));
  };

  const handleOnDragEnd = (sizes: number[]) => {
    dispatch(setPreferredSizes(sizes));
  };
  return (
    <div className="h-full overflow-auto bg-[#F4F4F4]">
      <Resizable ref={ref} vertical className="h-full" proportionalLayout={false} onDragEnd={handleOnDragEnd}>
        {accordions.map((accordion, index) => {
          return (
            <ResizablePanel
              key={accordion.id}
              minSize={accordion.isOpen ? 100 : 35}
              maxSize={accordion.isOpen ? Infinity : 35}
            >
              <Accordion title={accordion.title} isOpen={accordion.isOpen} handleClick={() => toggleAccordion(index)}>
                {getDesktopComponentByName(accordion.content as DesktopComponentsOmitted)}
              </Accordion>
            </ResizablePanel>
          );
        })}
        <ResizablePanel minSize={0} maxSize={Infinity}>
          <span />
        </ResizablePanel>
      </Resizable>
    </div>
  );
};
