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
import { DropdownMenu as DM, Icon, cn } from "@repo/ui";
import { OsType } from "@tauri-apps/plugin-os";
import { HTMLProps, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { ActionsGroup } from "../ActionsGroup";
import { DNDWrapper } from "./DNDWrapper";

interface WidgetBarProps extends HTMLProps<HTMLDivElement> {
  os: OsType;
}

const widgetsList = [
  {
    id: 1,
    label: "Alerts",
    icon: "HeadBarAlerts" as const,
    actions: ["1", "2"],
    defaultAction: false,
  },
  {
    id: 2,
    label: "Discovery",
    icon: "HeadBarDiscovery" as const,
    actions: ["1"],
    defaultAction: true,
  },
  {
    id: 3,
    label: "Community",
    icon: "HeadBarCommunity" as const,
    actions: ["1", "2"],
    defaultAction: true,
  },
];

export const WidgetBar = ({ os, className, ...props }: WidgetBarProps) => {
  const [draggedId, setDraggedId] = useState<UniqueIdentifier | null>(null);

  const [DNDlist, setDNDList] = useState<number[]>([]);
  const [overflownList, setOverflownList] = useState<number[]>(widgetsList.map((item) => item.id));

  const DNDListRef = useRef<HTMLDivElement>(null);
  const overflownListRef = useRef<HTMLDivElement>(null);

  function handleDragStart(event: DragStartEvent) {
    setDraggedId(event.active.id);
  }

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    setDraggedId(null);

    if (!over) return;

    if (active.id !== over.id) {
      setDNDList((items) => {
        const oldIndex = items.indexOf(active.id as number);
        const newIndex = items.indexOf(over.id as number);

        return arrayMove(items, oldIndex, newIndex);
      });
    }
  };

  const sensors = useSensors(
    useSensor(MouseSensor, {
      activationConstraint: {
        distance: 1,
      },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleVisibleList = (entries: IntersectionObserverEntry[]) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) return;

      const target = entry.target as HTMLElement;
      const targetId = Number(target.dataset.itemid);

      setDNDList((prevList) => {
        return prevList.filter((id) => id !== targetId);
      });
      setOverflownList((prevOverflownIds) => {
        if (prevOverflownIds.includes(targetId)) return prevOverflownIds;
        return [targetId, ...prevOverflownIds];
      });
    });
  };

  const handleOverflownList = (entries: IntersectionObserverEntry[]) => {
    entries.forEach((entry) => {
      if (!entry.isIntersecting) return;

      const target = entry.target as HTMLElement;
      const targetId = Number(target.dataset.itemid);

      setDNDList((prevList) => {
        if (prevList.includes(targetId)) return prevList;
        return [...prevList, targetId];
      });
      setOverflownList((prevOverflownIds) => {
        return prevOverflownIds.filter((id) => id !== targetId);
      });
    });
  };

  useEffect(() => {
    if (!DNDListRef.current || !overflownListRef.current) return;

    const visibleListObserver = new IntersectionObserver(handleVisibleList, {
      root: document.querySelector("header"),
      threshold: 0.99, // this is set to 0.99 instead of 1 because for some reason Linux always sees the last item as not intersecting
    });

    const overflownListObserver = new IntersectionObserver(handleOverflownList, {
      root: document.querySelector("header"),
      threshold: 0.99, // this is set to 0.99 instead of 1 because for some reason Linux always sees the last item as not intersecting
    });

    Array.from(DNDListRef.current.children).forEach((child) => {
      const element = child as HTMLElement;
      if (element.dataset.overflowable) visibleListObserver.observe(element);
    });

    Array.from(overflownListRef.current.children).forEach((child) => {
      const element = child as HTMLElement;
      if (element.dataset.overflowable) overflownListObserver.observe(element);
    });

    return () => {
      visibleListObserver.disconnect();
      overflownListObserver.disconnect();
    };
  }, [DNDlist, overflownList]);

  const OverflownMenu = ({
    classNameContent,
    classNameTrigger,
  }: {
    classNameContent?: string;
    classNameTrigger?: string;
  }) => {
    console.log({
      classNameContent,
      classNameTrigger,
    });
    return (
      <DM.Root>
        <DM.Trigger className={cn("DM.Trigger rounded p-[7px] transition-colors hover:bg-[#D3D3D3]", classNameTrigger)}>
          <Icon icon="ThreeHorizontalDots" className="flex size-4 items-center justify-center" />
        </DM.Trigger>

        <DM.Content className={cn("z-50 bg-white", classNameContent)}>
          {overflownList.map((id) => {
            const item = widgetsList.find((item) => id === item.id)!;
            return (
              <ActionsGroup
                icon={item.icon}
                label={item.label}
                actions={item.actions}
                defaultAction={item.defaultAction}
                key={item.id}
              />
            );
          })}
        </DM.Content>
      </DM.Root>
    );
  };

  return (
    <div className={cn("flex items-center gap-1", className)} {...props}>
      {os !== "macos" && <ActionsGroup icon="HeadBarSettingsWithNotification" iconClassName="size-[18px]" />}
      <div className="flex items-center gap-3">
        <ActionsGroup
          icon="HeadBarMossStudio"
          label="moss-studio"
          actions={["1", "2"]}
          iconClassName="size-[22px] -my-[4px]"
        />

        <div className="flex w-full items-center justify-start gap-1">
          {DNDlist.length === 0 && <OverflownMenu classNameTrigger="ml-1.5" />}
          <div className="sortable flex w-full items-center" ref={DNDListRef}>
            <DndContext
              sensors={sensors}
              collisionDetection={closestCenter}
              onDragEnd={handleDragEnd}
              onDragStart={handleDragStart}
            >
              <SortableContext items={DNDlist} strategy={horizontalListSortingStrategy}>
                {DNDlist.map((id, index) => {
                  const item = widgetsList.find((item) => item.id === id)!;
                  const shouldShowSelect =
                    overflownList.length > 0 && DNDlist.length !== 0 && index + 1 === DNDlist.length;

                  return (
                    <span
                      className="flex items-center gap-2"
                      data-overflowable={true}
                      data-itemid={item.id}
                      key={item.id}
                    >
                      <DNDWrapper sortableId={item.id} draggingClassName="z-50 cursor-grabbing opacity-50 shadow-2xl">
                        <ActionsGroup
                          icon={item.icon}
                          label={item.label}
                          actions={item.actions}
                          defaultAction={item.defaultAction}
                        />
                      </DNDWrapper>

                      {shouldShowSelect && <OverflownMenu />}
                    </span>
                  );
                })}
              </SortableContext>

              {draggedId
                ? createPortal(
                    <DragOverlay>
                      <ActionsGroup
                        icon={widgetsList.find((item) => item.id === draggedId)?.icon!}
                        label={widgetsList.find((item) => item.id === draggedId)?.label}
                        actions={widgetsList.find((item) => item.id === draggedId)?.actions!}
                        defaultAction={widgetsList.find((item) => item.id === draggedId)?.defaultAction}
                        className=" cursor-grabbing rounded border !border-[#c5c5c5] bg-[#D3D3D3] shadow-lg"
                      />
                    </DragOverlay>,
                    document.body
                  )
                : null}
            </DndContext>
          </div>
          <div className="overflown invisible flex" ref={overflownListRef}>
            {overflownList.map((id) => {
              const item = widgetsList.find((item) => item.id === id)!;
              return (
                <ActionsGroup
                  key={item.id}
                  icon={item.icon}
                  label={item.label}
                  actions={item.actions}
                  defaultAction={item.defaultAction}
                  data-overflowable={true}
                  data-itemid={item.id}
                />
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
};
