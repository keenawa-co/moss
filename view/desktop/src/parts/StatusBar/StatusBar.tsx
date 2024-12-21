import { useEffect, useRef, useState, type ComponentPropsWithoutRef } from "react";
import { createPortal } from "react-dom";

import { swapListById } from "@/utils/swapListById";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import {
  draggable,
  dropTargetForElements,
  monitorForElements,
} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";
import { cn, Icon, Icons } from "@repo/moss-ui";

import { DropIndicator } from "../../components/DropIndicator";

interface Item {
  id: number;
  icon: "StatusBarTerminal" | "StatusBarCommit" | "StatusBarSearch";
  label: string;
}

const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  const [DNDList, setDNDList] = useState<Item[]>([
    {
      id: 1,
      icon: "StatusBarTerminal",
      label: "Terminal",
    },
    {
      id: 2,
      icon: "StatusBarCommit",
      label: "12 Commit",
    },
    {
      id: 3,
      icon: "StatusBarSearch",
      label: "Search",
    },
  ]);

  useEffect(() => {
    return monitorForElements({
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) {
          return;
        }

        const sourceData = source.data;
        const targetData = target.data;
        if (!sourceData || !targetData) {
          return;
        }

        const updatedItems = swapListById(sourceData.id as number, targetData.id as number, DNDList);
        if (!updatedItems) {
          return;
        }

        setDNDList(updatedItems);
      },
    });
  }, [DNDList]);

  return (
    <footer
      className={cn(
        "flex h-[26px] w-screen justify-between pr-[26px] background-[--color-statusBar-background]",
        className
      )}
    >
      <div className="flex h-full">
        <StatusBarButton icon="StatusBarMacButton" className="bg-[#054ADA] px-[9px] py-[5px]" iconClassName="size-4" />

        <div className="flex h-full gap-1">
          {DNDList.map((item) => (
            <StatusBarButton key={item.id} {...item} isDraggable />
          ))}
        </div>
      </div>

      <div className="flex h-full gap-1">
        <StatusBarButton icon="StatusBarGitlens" label="2 weeks ago, you" />
        <StatusBarButton label="UTF-8" />
        <StatusBarButton label="24 Ln, 16 Col" />
        <StatusBarButton label="4 Spaces" />
        <StatusBarButton label="Rust" />

        <div className="group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-opacity-10 focus:bg-white focus:bg-opacity-10">
          <StatusCircle className="size-[6px] bg-[#D62A18]" />
          <span>2 Errors</span>
        </div>

        <div className="group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-opacity-10 focus:bg-white focus:bg-opacity-10">
          <StatusCircle className="size-[6px] bg-[#FFC505]" />
          <span>15 Warnings</span>
        </div>

        <StatusBarButton label="--READ--" />
      </div>
    </footer>
  );
};

interface StatusBarButtonProps extends Omit<ComponentPropsWithoutRef<"button">, "id"> {
  icon?: Icons;
  label?: string;
  className?: string;
  iconClassName?: string;

  id?: number;
  isDraggable?: boolean;
}

const StatusCircle = ({ className }: { className?: string }) => {
  return <div className={cn("flex items-center justify-center rounded-full", className)} />;
};

const StatusBarButton = ({
  id,
  icon,
  iconClassName,
  label,
  className,
  isDraggable,
  ...props
}: StatusBarButtonProps) => {
  const ref = useRef<HTMLButtonElement | null>(null);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  useEffect(() => {
    const element = ref.current;

    if (!element || !isDraggable) return;

    return combine(
      draggable({
        element: element,
        getInitialData: () => ({ id, icon, label }),
        onDrop: () => {
          setPreview(null);
        },
        onGenerateDragPreview({ nativeSetDragImage }) {
          setCustomNativeDragPreview({
            nativeSetDragImage,
            render({ container }) {
              setPreview((prev) => (prev === container ? prev : container));
            },
          });
        },
      }),
      dropTargetForElements({
        element,
        onDrop: ({ source, self }) => {
          if (source.data?.id === self.data.id) return;

          setClosestEdge(null);
        },
        getData({ input }) {
          return attachClosestEdge(
            { id, label, icon },
            {
              element,
              input,
              allowedEdges: ["right", "left"],
            }
          );
        },
        getIsSticky() {
          return true;
        },
        onDragEnter({ self }) {
          const closestEdge = extractClosestEdge(self.data);
          setClosestEdge(closestEdge);
        },
        onDrag({ self }) {
          const closestEdge = extractClosestEdge(self.data);

          setClosestEdge((current) => {
            if (current === closestEdge) {
              return current;
            }
            return closestEdge;
          });
        },
        onDragLeave() {
          setClosestEdge(null);
        },
      })
    );
  }, [id, label, isDraggable, icon]);

  return (
    <button
      ref={ref}
      {...props}
      className={cn(
        "group relative flex h-full items-center gap-1 px-2 text-white transition hover:bg-[rgb(39,114,255)]",
        className
      )}
    >
      {icon && <Icon className={cn("size-[18px]", iconClassName)} icon={icon} />}
      {label && <span className="text-sm">{label}</span>}
      {closestEdge ? <DropIndicator edge={closestEdge} gap={4} /> : null}
      {preview && createPortal(<StatusBarButton icon={icon} label={label} className="bg-sky-500" />, preview)}
    </button>
  );
};

export default StatusBar;
