import { forwardRef, useState, type ComponentPropsWithoutRef } from "react";
import { createPortal } from "react-dom";

import {
  closestCenter,
  DndContext,
  DragEndEvent,
  DragOverlay,
  DragStartEvent,
  MouseSensor,
  UniqueIdentifier,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import { arrayMove, horizontalListSortingStrategy, SortableContext } from "@dnd-kit/sortable";
import { cn, Icon, Icons } from "@repo/moss-ui";

import { SortableDNDWrapper } from "./SortableDNDWrapper";

interface Item {
  id: UniqueIdentifier;
  icon: "StatusBarTerminal" | "StatusBarCommit" | "StatusBarSearch";
  label: string;
}

const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  const [draggedItem, setDraggedItem] = useState<Item | null>(null);

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

  const handleDragStart = (event: DragStartEvent) => {
    setDraggedItem(DNDList.find((i) => i.id === event.active.id)!);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    setDraggedItem(null);

    if (!over) return;

    if (active.id !== over.id) {
      setDNDList((items) => {
        const oldIndex = items.findIndex((i) => i.id === active.id);
        const newIndex = items.findIndex((i) => i.id === over.id);

        return arrayMove(items, oldIndex, newIndex);
      });
    }
  };

  const sensors = useSensors(
    useSensor(MouseSensor, {
      activationConstraint: {
        distance: 1,
      },
    })
  );

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
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={handleDragEnd}
            onDragStart={handleDragStart}
          >
            <SortableContext items={DNDList} strategy={horizontalListSortingStrategy}>
              {DNDList.map((item) => {
                return (
                  <SortableDNDWrapper
                    key={item.id}
                    id={item.id}
                    draggingClassName="z-50 cursor-grabbing opacity-50 shadow-2xl"
                  >
                    <StatusBarButton key={item.id} icon={item.icon} label={item.label} />
                  </SortableDNDWrapper>
                );
              })}
            </SortableContext>

            {draggedItem && <DraggedComponent item={draggedItem} />}
          </DndContext>
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

export default StatusBar;

interface StatusBarButtonProps extends ComponentPropsWithoutRef<"button"> {
  icon?: Icons;
  label?: string;
  className?: string;
  iconClassName?: string;
}

const StatusCircle = ({ className }: { className?: string }) => {
  return <div className={cn("flex items-center justify-center rounded-full", className)} />;
};

const StatusBarButton = forwardRef<HTMLButtonElement, StatusBarButtonProps>(
  ({ icon, iconClassName, label, className, ...props }, ref) => {
    return (
      <button
        ref={ref}
        {...props}
        className={cn(
          "group flex h-full items-center gap-1 px-2 text-white transition hover:bg-[rgb(39,114,255)]",
          className
        )}
      >
        {icon && <Icon className={cn("size-[18px]", iconClassName)} icon={icon} />}
        {label && <span className="text-sm">{label}</span>}
      </button>
    );
  }
);

const DraggedComponent = ({ item }: { item: Item }) => {
  return createPortal(
    <DragOverlay>
      <StatusBarButton
        icon={item.icon as Icons}
        label={item.label}
        className="cursor-grabbing bg-[rgb(39,114,255)] text-white shadow-lg transition hover:opacity-100 focus:opacity-100"
      />
    </DragOverlay>,
    document.body
  );
};
