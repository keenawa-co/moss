import { useRef, useEffect, useState } from "react";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

type HoveredState = "idle" | "validMove" | "invalidMove";

const getColor = (state: HoveredState): string => {
  if (state === "validMove") {
    return "lightgreen";
  } else if (state === "invalidMove") {
    return "pink";
  }
  return "none";
};

const DraggableAccordion = ({
  title,
  isOpen = false,
  location,
  handleClick,
  children,
}: {
  title: string;
  isOpen?: boolean;
  location: number;
  handleClick: () => void;
  children: React.ReactNode[] | React.ReactNode;
}) => {
  const ref = useRef(null);
  const dropRef = useRef(null);

  const [dragging, setDragging] = useState<boolean>(false);
  const [state, setState] = useState<HoveredState>("idle");

  useEffect(() => {
    const el = ref.current;

    if (!el) return;

    return draggable({
      element: el,
      getInitialData: () => ({ location, isOpen }),
      onDragStart: () => setDragging(true),
      onDrop: () => setDragging(false),
    });
  }, [location, isOpen]);

  useEffect(() => {
    const el = dropRef.current;
    if (!el) return;

    return dropTargetForElements({
      element: el,
      getData: () => ({ location }),
      onDragEnter: ({ source }) => {
        const targetIndex = location;
        const draggedIndex = source.data.index;

        if (targetIndex === draggedIndex) setState("invalidMove");
        else setState("validMove");
      },
      onDragLeave: () => setState("idle"),
      onDrop: () => setState("idle"),
    });
  }, [location]);

  return (
    <div key={location} style={{ background: getColor(state) }} ref={dropRef}>
      <div ref={ref} onClick={handleClick} style={{ opacity: dragging ? 0.4 : 1 }}>
        {title}
      </div>
      <div className={isOpen ? "text-gray-500 text-xs" : "visually-hidden"}>{children}</div>
    </div>
  );
};

export default DraggableAccordion;
