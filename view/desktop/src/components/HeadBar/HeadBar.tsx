import { HeadBarButton } from "./HeadBarButton";
import { type } from "@tauri-apps/plugin-os";
import { cn, Icon } from "@repo/ui";
import {
  closestCenter,
  DndContext,
  DragEndEvent,
  DragOverlay,
  DragStartEvent,
  KeyboardSensor,
  PointerSensor,
  UniqueIdentifier,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  arrayMove,
  horizontalListSortingStrategy,
  SortableContext,
  sortableKeyboardCoordinates,
} from "@dnd-kit/sortable";
import { useState } from "react";
import { RootState, useAppDispatch } from "@/store";
import { toggleSidebarVisibility } from "@/store/sidebar/sidebarSlice";
import { useSelector } from "react-redux";
import { createPortal } from "react-dom";
import { Controls } from "./Controls/Controls";

export const HeadBar = () => {
  const dispatch = useAppDispatch();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);

  let os = type();

  // os = "macos";
  // os = "linux";

  const [activeId, setActiveId] = useState<UniqueIdentifier | null>(null);
  const [isSorting, setSorting] = useState(false);

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
    console.log(event);
    setActiveId(event.active.id);
    setSorting(true);
  }

  const handleDragEnd = (event: DragEndEvent) => {
    setSorting(false);
    const { active, over } = event;
    setActiveId(null);

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
    <header data-tauri-drag-region className={cn("flex h-full bg-[#E0E0E0] shadow-[inset_0_-1px_0_0_#C6C6C6]")}>
      {os === "macos" && <Controls os={os} />}
      <div className="flex w-full items-center py-[3px]">
        <div
          className={cn("flex w-full items-center justify-between", {
            "pr-[12px]": os === "macos",
            "px-[16px]": os === "windows" || os === "linux",
          })}
          data-tauri-drag-region
        >
          <div className="flex items-center">
            <button className="flex w-max items-center rounded px-[10px] py-[5px] transition-colors hover:bg-[#C6C6C6]">
              <Icon icon="HeadBarMossStudio" className="mr-1.5 size-[22px] text-[#525252]" />
              <span className="mr-[2px] w-max font-medium text-[#161616]">moss-studio</span>
              <Icon icon="ArrowheadDown" className="text-[#525252]" />
            </button>

            <Separator />

            <div className="flex w-full justify-between">
              <div
                className={cn("flex items-center gap-2 overflow-hidden", {
                  "rounded bg-[#c8c8c8]": isSorting,
                })}
              >
                <DndContext
                  sensors={sensors}
                  collisionDetection={closestCenter}
                  onDragEnd={handleDragEnd}
                  onDragStart={handleDragStart}
                >
                  <SortableContext items={items} strategy={horizontalListSortingStrategy}>
                    {items.map((item) => (
                      <HeadBarButton
                        key={item.id}
                        sortableId={item.id}
                        icon={item.icon}
                        label={item.label}
                        className={cn("text-ellipsis px-[10px] py-[5px]")}
                      />
                    ))}
                  </SortableContext>

                  {activeId
                    ? createPortal(
                        <DragOverlay>
                          <HeadBarButton
                            className="cursor-grabbing !bg-[#C6C6C6]  px-[10px] py-[5px] shadow-lg"
                            icon={items.find((item) => item.id === activeId)?.icon!}
                            label={items.find((item) => item.id === activeId)?.label}
                          />
                        </DragOverlay>,
                        document.body
                      )
                    : null}
                </DndContext>
              </div>
            </div>
          </div>
          <div className="flex items-center gap-4">
            <div className="flex items-center">
              <button className="flex items-center gap-[1px] transition-colors">
                <div className="flex h-full items-center gap-[6px] rounded py-[5px] pl-[10px] pr-[8px] hover:bg-[#C6C6C6] ">
                  <Icon icon="HeadBarBranch" className="size-[18px] text-[#525252]" />
                  <div className="flex items-center gap-[2px]">
                    <span className=" leading-4 text-[#161616]">main</span>
                    <span className="rounded bg-[#C6C6C6] px-1 text-xs font-semibold text-[#525252]">#50</span>
                  </div>
                </div>

                <div className="flex items-center gap-1 pr-[10px]">
                  <Icon icon="HeadBarBranchSuccess" className="size-[16px] rounded" />
                  <Icon icon="HeadBarBranchRefresh" className="size-[16px] rounded text-[#525252] hover:bg-[#C6C6C6]" />
                  <Icon icon="ArrowheadDown" className="size-[16px] rounded text-[#525252] hover:bg-[#C6C6C6]" />
                </div>
              </button>
            </div>

            <div className="flex items-center gap-1.5">
              <HeadBarButton
                icon={isSidebarVisible ? "HeadBarPrimarySideBarActive" : "HeadBarPrimarySideBar"}
                className="p-[3px]"
                iconClassName="w-[16px] h-[14px]"
                onClick={() => dispatch(toggleSidebarVisibility(0))}
              />
              <HeadBarButton icon="HeadBarPanelActive" className="p-[3px]" iconClassName="w-[16px] h-[14px]" />
              <HeadBarButton icon="HeadBarSecondarySideBar" className="p-[3px]" iconClassName="w-[16px] h-[14px]" />
              <HeadBarButton icon="HeadBarCustomizeLayout" className="p-[3px]" iconClassName="w-[16px] h-[14px]" />
            </div>

            <Separator />

            <div className="flex items-center gap-3">
              <HeadBarButton icon="HeadBarAccount" className="p-[2px]" iconClassName="size-[18px]" />
              <HeadBarButton icon="HeadBarNotifications" className="p-[2px]" iconClassName="size-[18px]" />
              <HeadBarButton icon="HeaderBarSettingsWithNotifictaion" className="p-[2px]" iconClassName="size-[18px]" />
            </div>
          </div>
        </div>
      </div>
      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};

const Separator = () => <div className="separator h-[15px] w-px bg-[#C6C6C6]" />;
