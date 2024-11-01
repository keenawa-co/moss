import { WindowControls } from "../window-controls/WindowControls";
import { HeadBarButton } from "./HeadBarButton";
import { type } from "@tauri-apps/plugin-os";
import { HeadBarDropdown } from "./HeadBarDropdown";
import { cn } from "@repo/ui";
import {
  closestCenter,
  DndContext,
  DragEndEvent,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import { arrayMove, SortableContext, sortableKeyboardCoordinates } from "@dnd-kit/sortable";
import { useState } from "react";

export const HeadBar = () => {
  let os = type();

  // os = "macos";
  // os = "linux";

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

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

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
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );
  return (
    <header
      data-tauri-drag-region
      className={cn("flex h-full max-h-[46px] items-center border-b border-solid border-[#C6C6C6] bg-[#E0E0E0]")}
    >
      {os === "macos" && <WindowControls platform={os} />}

      <div
        className={cn("flex grow items-center", {
          "pl-[10px] pr-[16px]": os === "macos",
          "px-[16px]": os === "windows" || os === "linux",
        })}
      >
        <HeadBarDropdown icon="HeadBarMossStudio" label="moss-studio" />

        <Separator />

        <div className="flex w-full justify-between" data-tauri-drag-region>
          <div className="flex items-center gap-4">
            <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
              <SortableContext items={items}>
                {items.map((item) => (
                  <HeadBarButton key={item.id} sortableId={item.id} icon={item.icon} label={item.label} />
                ))}
              </SortableContext>
            </DndContext>
          </div>

          <div className="flex items-center gap-4">
            <HeadBarDropdown icon="HeadBarBranch" label="moss" />
            <HeadBarButton icon="HeadBarTogglePrimarySideBar" />
            <HeadBarButton icon="HeadBarTogglePanel" />
            <HeadBarButton icon="HeadBarToggleSecondarySideBar" />
            <HeadBarButton icon="HeadBarCustomizeLayout" />
          </div>
        </div>

        <Separator />

        <div className="flex items-center gap-4">
          <HeadBarButton icon="HeadBarAccount" />
          <HeadBarButton icon="HeadBarNotifications" />
          <HeadBarButton icon="HeadBarSettings" />
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <WindowControls platform={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <WindowControls />}
    </header>
  );
};

const Separator = () => <div className="separator mx-3 h-[15px] w-px bg-[#C6C6C6]" />;
