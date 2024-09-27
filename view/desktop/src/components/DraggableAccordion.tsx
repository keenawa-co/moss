import { useRef, useEffect, useState } from "react";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { Icon } from "@repo/ui";
import { cn } from "@/utils";
import { DropIndicator } from "@atlaskit/pragmatic-drag-and-drop-react-drop-indicator/box";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { attachClosestEdge, extractClosestEdge, Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
type HoveredState = "idle" | "valid" | "invalid";

interface DraggableAccordionProps {
  title: string;
  isOpen?: boolean;
  location: number;
  handleClick: () => void;
  children: React.ReactNode[] | React.ReactNode;
}

const DraggableAccordion = ({ title, isOpen = false, location, handleClick, children }: DraggableAccordionProps) => {
  const ref = useRef(null);
  const dropRef = useRef(null);

  const [dragging, setDragging] = useState<boolean>(false);
  const [state, setState] = useState<HoveredState>("idle");
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

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

    return combine(
      dropTargetForElements({
        element: el,
        getData: () => ({ location }),
        onDragStart: () => setClosestEdge("bottom"),
        onDragEnter: ({ source }) => {
          console.log({ source });
          setClosestEdge("bottom");

          const targetIndex = location;
          const draggedIndex = source.data.location;

          if (targetIndex == draggedIndex) setState("invalid");
          else setState("valid");
        },
        onDragLeave: () => {
          setState("idle");
          setClosestEdge(null);
        },
        onDrop: () => {
          setState("idle");
          setClosestEdge(null);
        },
      }),
      dropTargetForElements({
        element: el,
        getData({ input, element }) {
          return attachClosestEdge(
            { location },
            {
              element,
              input,
              allowedEdges: ["top", "bottom"],
            }
          );
        },
        onDrag({ self }) {
          const closestEdge = extractClosestEdge(self.data);
          setClosestEdge(closestEdge);
        },
        onDragLeave() {
          setClosestEdge(null);
        },
        onDrop() {
          setClosestEdge(null);
        },
      })
    );
  }, [location]);

  return (
    <div
      ref={dropRef}
      key={location}
      className={cn("relative", {
        "bg-stone-300": state == "valid",
        "bg-red-300": state == "invalid",
      })}
    >
      <div
        ref={ref}
        onClick={handleClick}
        style={{ opacity: dragging ? 0.4 : 1 }}
        className="flex items-center py-[5px] px-2"
      >
        <div className={cn(`size-5 flex items-center justify-center cursor-pointer`, { "rotate-90": isOpen })}>
          <Icon icon="ArrowRight" className="text-xs" />
        </div>
        <span className="font-bold">{title}</span>
      </div>

      <div className={isOpen ? "text-gray-500 text-xs pl-6" : "visually-hidden"}>{children}</div>
      {closestEdge && <DropIndicator edge={closestEdge} />}
    </div>
  );
};

export default DraggableAccordion;
