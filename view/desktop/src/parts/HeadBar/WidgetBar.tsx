import { HTMLProps, useEffect, useRef, useState } from "react";

import { ActionsGroup } from "@/components/ActionsGroup";
import { swapListByIndex } from "@/utils/swapListByIndex";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { cn, DropdownMenu as DM, Icon } from "@repo/moss-ui";
import { OsType } from "@tauri-apps/plugin-os";

interface WidgetBarProps extends HTMLProps<HTMLDivElement> {
  os: OsType;
}

// FIXME: remove constant widgetsList
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
  const DNDListRef = useRef<HTMLDivElement>(null);
  const [DNDList, setDNDList] = useState<number[]>([]);

  useEffect(() => {
    return monitorForElements({
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) return;

        const sourceData = source.data;
        const targetData = target.data;
        if (!sourceData || !targetData) return;

        const sourceIndex = DNDList.indexOf(sourceData.id as number);
        const targetIndex = DNDList.indexOf(targetData.id as number);

        const updatedItems = swapListByIndex(sourceIndex, targetIndex, DNDList);
        if (!updatedItems) return;

        setDNDList(updatedItems);
      },
    });
  }, [DNDList]);

  // overflownList

  const overflownListRef = useRef<HTMLDivElement>(null);
  const [overflownList, setOverflownList] = useState<number[]>(widgetsList.map((item) => item.id));

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
  }, [DNDList, overflownList]);

  const OverflownMenu = ({
    classNameContent,
    classNameTrigger,
  }: {
    classNameContent?: string;
    classNameTrigger?: string;
  }) => {
    return (
      <DM.Root>
        <DM.Trigger className={cn("DM.Trigger rounded p-[7px] transition-colors hover:bg-[#D3D3D3]", classNameTrigger)}>
          <Icon icon="ThreeHorizontalDots" className="flex size-4 items-center justify-center" />
        </DM.Trigger>

        <DM.Content className={cn("z-50 flex flex-col gap-0.5 bg-white", classNameContent)} align="start">
          {overflownList.map((id) => {
            const item = widgetsList.find((item) => id === item.id)!;
            return <DM.Item label={item.label} icon={item.icon} key={item.id} iconClassName="size-[15px]" />;
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
          {DNDList.length === 0 && <OverflownMenu />}
          <div className="flex w-full items-center" ref={DNDListRef}>
            {DNDList.map((id, index) => {
              const item = widgetsList.find((item) => item.id === id)!;
              const shouldShowSelect = overflownList.length > 0 && DNDList.length !== 0 && index + 1 === DNDList.length;

              return (
                <span className="flex items-center gap-2" data-overflowable={true} data-itemid={item.id} key={item.id}>
                  <ActionsGroup
                    icon={item.icon}
                    label={item.label}
                    actions={item.actions}
                    defaultAction={item.defaultAction}
                    isDraggable
                    id={item.id}
                  />

                  {shouldShowSelect && <OverflownMenu />}
                </span>
              );
            })}
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
