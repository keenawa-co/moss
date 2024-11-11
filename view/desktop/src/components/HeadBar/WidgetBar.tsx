import {
  DndContext,
  DragEndEvent,
  DragOverlay,
  DragStartEvent,
  KeyboardSensor,
  MouseSensor,
  UniqueIdentifier,
  closestCenter,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  SortableContext,
  arrayMove,
  horizontalListSortingStrategy,
  sortableKeyboardCoordinates,
} from "@dnd-kit/sortable";
import { Icon, cn } from "@repo/ui";
import { OsType } from "@tauri-apps/plugin-os";
import { HTMLProps, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { HeadBarButton } from "./HeadBarButton";
import { ContextMenu } from "@repo/ui";

interface WidgetBarProps extends HTMLProps<HTMLDivElement> {
  os: OsType;
}

export const WidgetBar = ({ os, className, ...props }: WidgetBarProps) => {
  const [draggedId, setDraggedId] = useState<UniqueIdentifier | null>(null);

  const [items, setItems] = useState([
    {
      id: 1,
      label: "Alerts",
      icon: "HeadBarAlerts" as const,
    },
    {
      id: 2,
      label: "Discovery",
      icon: "HeadBarDiscovery" as const,
    },
    {
      id: 3,
      label: "Community",
      icon: "HeadBarCommunity" as const,
    },
  ]);

  function handleDragStart(event: DragStartEvent) {
    setDraggedId(event.active.id);
  }

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    setDraggedId(null);

    if (!over) return;

    if (active.id !== over.id) {
      setItems((items) => {
        const oldIndex = items.findIndex((a) => a.id === active.id);
        const newIndex = items.findIndex((a) => a.id === over.id);

        return arrayMove(items, oldIndex, newIndex);
      });
    }
  };

  const sensors = useSensors(
    useSensor(MouseSensor, {
      activationConstraint: {
        distance: 5,
      },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const [overflownItemsIds, setOverflownItemsIds] = useState<number[]>([]);

  const DNDListRef = useRef<HTMLDivElement>(null);
  const handleIntersection = (entries: IntersectionObserverEntry[]) => {
    setOverflownItemsIds((prevOverflownIds) => {
      const updatedOverflownIds = [...prevOverflownIds];

      entries.forEach((entry) => {
        const targetId = Number(entry.target.dataset.itemid);

        if (!entry.isIntersecting) {
          entry.target.classList.add("invisible", "pointer-events-none", "touch-none");
          //@ts-ignore
          entry.target.disabled = true;

          if (!updatedOverflownIds.includes(targetId)) {
            updatedOverflownIds.push(targetId);
          }
        } else {
          entry.target.classList.remove("invisible", "pointer-events-none", "touch-none");
          //@ts-ignore
          entry.target.disabled = false;

          const index = updatedOverflownIds.indexOf(targetId);
          if (index !== -1) {
            updatedOverflownIds.splice(index, 1);
          }
        }
      });

      return updatedOverflownIds;
    });
  };

  useEffect(() => {
    if (!DNDListRef.current) return;

    const observer = new IntersectionObserver(handleIntersection, {
      root: document.querySelector("header"),
      threshold: 1,
    });

    Array.from(DNDListRef.current.children).forEach((item) => {
      //@ts-ignore
      if (item.dataset.listitem) observer.observe(item);
    });

    return () => {
      observer.disconnect();
    };
  }, [DNDListRef, items]);

  const OverflownMenu = ({ classNameContent, classNameTrigger, ...props }: any) => {
    const reversedList = [...overflownItemsIds].reverse();

    return (
      <ContextMenu.Root {...props}>
        <ContextMenu.Trigger className={classNameTrigger}>
          <Icon icon="ThreeHorizontalDots" />
        </ContextMenu.Trigger>
        <ContextMenu.Content className={cn("flex flex-col items-start z-100", classNameContent)}>
          {reversedList.map((id) => {
            return (
              <button className="rounded px-2 hover:bg-stone-300">{items.find((item) => id === item.id)?.label}</button>
            );
          })}
        </ContextMenu.Content>
      </ContextMenu.Root>
    );
  };

  return (
    <div className={cn("flex items-center gap-1", className)} {...props}>
      {os !== "macos" && (
        <HeadBarButton
          icon="HeadBarSettingsWithNotification"
          className="flex size-[30px] items-center justify-center px-2"
          iconClassName="size-[18px]"
        />
      )}
      <div className="flex items-center gap-3">
        <button className="flex h-[30px] w-max items-center rounded pl-2.5 pr-1 transition-colors hover:bg-[#D3D3D3]">
          <Icon icon="HeadBarMossStudio" className="mr-1.5 size-[22px] text-[#525252]" />
          <span className="mr-0.5 w-max text-[#161616]">moss-studio</span>
          <Icon icon="ArrowheadDown" className="text-[#525252]" />
        </button>

        <div className=" flex w-full items-center justify-start gap-1" ref={DNDListRef}>
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={handleDragEnd}
            onDragStart={handleDragStart}
          >
            <SortableContext items={items} strategy={horizontalListSortingStrategy}>
              {items.map((item, index) => (
                <>
                  {items.length === overflownItemsIds.length && index === 0 && (
                    <OverflownMenu classNameTrigger="pl-5" key={`OverflowMenuAtStart-${index}`} />
                  )}
                  <span className="flex items-center" data-listItem={true} data-itemId={item.id}>
                    <HeadBarButton
                      key={item.id}
                      sortableId={item.id}
                      icon={item.icon}
                      label={item.label}
                      className={cn("h-[30px] text-ellipsis px-2")}
                    />
                    {overflownItemsIds.length > 0 && items.length - overflownItemsIds.length === index + 1 && (
                      <OverflownMenu key={`OverflowMenuBetweenMenuItems-${index}`} />
                    )}
                  </span>
                </>
              ))}
            </SortableContext>

            {draggedId
              ? createPortal(
                  <DragOverlay>
                    <HeadBarButton
                      className="h-[30px] cursor-grabbing !bg-[#e0e0e0] px-2 shadow-lg"
                      icon={items.find((item) => item.id === draggedId)?.icon!}
                      label={items.find((item) => item.id === draggedId)?.label}
                    />
                  </DragOverlay>,
                  document.body
                )
              : null}
          </DndContext>
        </div>
      </div>
    </div>
  );
};
