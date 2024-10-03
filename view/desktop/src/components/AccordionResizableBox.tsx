import { useAppDispatch } from "@/store";
import { selectAccordionById } from "@/store/accordion/accordionSelectors";
import { updateAccordionById } from "@/store/accordion/accordionSlice";
import { Resizable } from "re-resizable";
import { ForwardedRef, forwardRef, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";

const pxStringToNumber = (pxString: string) => {
  return parseFloat(pxString.replace("px", ""));
};

const AccordionResizableBox = forwardRef(
  (
    { accordionId, isOpen, children, ...props }: { accordionId: number; isOpen: boolean; children: React.ReactNode },
    ref: ForwardedRef<HTMLDivElement>
  ) => {
    const dispatch = useAppDispatch();
    const [height, setHeight] = useState(35);
    const openedHeight = useRef(height);
    const accordionItem = useSelector(selectAccordionById(accordionId));

    useEffect(() => {
      if (!isOpen) setHeight(35);
      else setHeight(openedHeight.current);
    }, [isOpen]);

    useEffect(() => {
      if (accordionItem) setHeight(accordionItem.preferredHeight || height);
    }, []);

    return (
      <div ref={ref} {...props}>
        <Resizable
          className="overflow-hidden border-b"
          minHeight={isOpen ? 100 : 35}
          maxHeight={400}
          size={{
            width: "100%",
            height: height,
          }}
          onResizeStop={(e, direction, ref, d) => {
            setHeight(pxStringToNumber(ref.style.height));
            openedHeight.current = pxStringToNumber(ref.style.height);
            dispatch(
              updateAccordionById({ id: accordionId, changes: { preferedHeight: pxStringToNumber(ref.style.height) } })
            );
          }}
          handleStyles={{
            top: {
              display: "none",
            },
            right: {
              display: "none",
            },
            left: {
              display: "none",
            },
          }}
        >
          {children}
        </Resizable>
      </div>
    );
  }
);

export default AccordionResizableBox;
