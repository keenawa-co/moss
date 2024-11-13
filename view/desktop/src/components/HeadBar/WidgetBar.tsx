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

  const [DNDItems, setDNDItems] = useState([
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
      setDNDItems((items) => {
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

  const [overflownDNDItemsIds, setOverflownDNDItemsIds] = useState<number[]>([]);

  const DNDListRef = useRef<HTMLDivElement>(null);
  const handleIntersection = (entries: IntersectionObserverEntry[]) => {
    setOverflownDNDItemsIds((prevOverflownIds) => {
      const updatedOverflownIds = [...prevOverflownIds];

      entries.forEach((entry) => {
        const target = entry.target as HTMLElement;
        const targetId = Number(target.dataset.itemid);

        console.log(entry.intersectionRatio);

        if (!entry.isIntersecting) {
          target.classList.add("invisible", "pointer-events-none", "touch-none");
          if (target instanceof HTMLButtonElement) {
            target.disabled = true;
          }

          if (!updatedOverflownIds.includes(targetId)) {
            updatedOverflownIds.push(targetId);
          }
        } else {
          target.classList.remove("invisible", "pointer-events-none", "touch-none");
          if (target instanceof HTMLButtonElement) {
            target.disabled = false;
          }

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
      threshold: 0.99, // this is set to 0.99 because for some reason it doesn't work with 1 on linux
    });

    Array.from(DNDListRef.current.children).forEach((child) => {
      const element = child as HTMLElement;
      if (element.dataset.listitem) observer.observe(element);
    });

    return () => {
      observer.disconnect();
    };
  }, [DNDListRef, DNDItems]);

  const OverflownMenu = ({ classNameContent, classNameTrigger, ...props }: any) => {
    const reversedList = [...overflownDNDItemsIds].reverse();

    // TODO replace with a DropdownMenu
    return (
      <ContextMenu.Root {...props}>
        <ContextMenu.Trigger className={classNameTrigger}>
          <button className="rounded p-[7px] transition-colors hover:bg-[#D3D3D3]">
            <Icon icon="ThreeHorizontalDots" className="flex size-4 items-center justify-center" />
          </button>
        </ContextMenu.Trigger>
        <ContextMenu.Content className={cn("flex flex-col items-start z-100", classNameContent)}>
          {reversedList.map((id) => {
            return <ContextMenu.Item label={DNDItems.find((item) => id === item.id)?.label || ""} hideIcon />;
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

        <div className="flex w-full items-center justify-start gap-1" ref={DNDListRef}>
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={handleDragEnd}
            onDragStart={handleDragStart}
          >
            <SortableContext items={DNDItems} strategy={horizontalListSortingStrategy}>
              {DNDItems.map((item, index) => (
                <>
                  {DNDItems.length === overflownDNDItemsIds.length && index === 0 && (
                    <OverflownMenu classNameTrigger="pl-[14px]" key={`OverflowMenuAtStart-${index}`} />
                  )}
                  <span className="flex items-center gap-2" data-listItem={true} data-itemId={item.id}>
                    <HeadBarButton
                      key={item.id}
                      sortableId={item.id}
                      icon={item.icon}
                      label={item.label}
                      className={cn("h-[30px] text-ellipsis px-2")}
                    />
                    {overflownDNDItemsIds.length > 0 && DNDItems.length - overflownDNDItemsIds.length === index + 1 && (
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
                      icon={DNDItems.find((item) => item.id === draggedId)?.icon!}
                      label={DNDItems.find((item) => item.id === draggedId)?.label}
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
