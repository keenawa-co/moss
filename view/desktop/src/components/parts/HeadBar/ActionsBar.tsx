import { RootState, useAppDispatch } from "@/store";
import { toggleSidebarVisibility } from "@/store/sidebar/sidebarSlice";
import { cn, Icon } from "@repo/moss-ui";
import { HTMLProps, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { invokeIpc } from "@/lib/backend/tauri";
import { MenuItem } from "@repo/desktop-models";
import { ActionButton } from "@/components/Action/ActionButton";
import { ActionsSubmenu } from "@/components/Action/ActionsSubmenu";
import { ActionsGroup } from "@/components/ActionsGroup";

export const ActionsBar = ({ className, ...props }: HTMLProps<HTMLDivElement>) => {
  const dispatch = useAppDispatch();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);
  const [activities, setActivities] = useState<MenuItem[]>([]);

  useEffect(() => {
    const getAllActivities = async () => {
      try {
        const res = await invokeIpc<MenuItem[], Error>("get_menu_items_by_namespace", { namespace: "headItem" }); // this here should be a type

        if (res.status === "ok") {
          setActivities(res.data);
          // console.log(res.data);
        }
      } catch (err) {
        console.error("Failed to get workbench state:", err);
      }
    };
    getAllActivities();
  }, []);

  return (
    <div className={cn("flex items-center gap-3", className)} {...props}>
      <div className="flex items-center">
        <button className="flex items-center gap-px transition-colors">
          <div className="flex h-full items-center gap-1.5 rounded py-1.5 pl-2.5 pr-2 hover:bg-[#D3D3D3]">
            <Icon icon="HeadBarBranch" className="size-[18px] text-[#525252]" />
            <div className="flex items-center gap-0.5">
              <span className="leading-4 text-[#161616]">main</span>
              <span className="rounded bg-[#C6C6C6] px-1 text-xs font-semibold text-[#525252]">#50</span>
            </div>
          </div>

          <div className="flex cursor-default items-center gap-1 pr-2.5">
            <Icon icon="HeadBarBranchSuccess" className="size-4 rounded" />
            <Icon icon="HeadBarBranchRefresh" className="size-4 rounded text-[#525252]" />
            <Icon icon="ArrowheadDown" className="size-4 rounded text-[#525252]" />
          </div>
        </button>
      </div>

      <div className="flex items-center">
        {activities.map((item, index) => {
          const icons = [
            "HeadBarPrimarySideBar",
            "HeadBarPanelActive",
            "HeadBarSecondarySideBar",
            "HeadBarCustomizeLayout",
          ];
          console.log(item);

          if ("action" in item) {
            const command = item.action.command;

            if (index === 0) {
              return (
                <ActionButton
                  key={command.id}
                  iconClassName="size-[18px]"
                  {...command}
                  icon={isSidebarVisible ? "HeadBarPrimarySideBarActive" : "HeadBarPrimarySideBar"}
                  onClick={() => dispatch(toggleSidebarVisibility({}))}
                  visibility={item.action.visibility}
                />
              );
            }
            return (
              <ActionButton
                key={command.id}
                iconClassName="size-[18px]"
                {...command}
                icon={icons[index]}
                visibility="compact"
              />
            );
          }

          if ("submenu" in item) {
            return (
              <ActionsSubmenu
                key={item.submenu.submenuId}
                iconClassName="size-[18px]"
                {...item.submenu}
                icon="HeadBarCustomizeLayout"
              />
            );
          }
        })}
        {/* <ActionsGroup
          icon={isSidebarVisible ? "HeadBarPrimarySideBarActive" : "HeadBarPrimarySideBar"}
          onClick={() => dispatch(toggleSidebarVisibility({}))}
          iconClassName="size-[18px]"
          className="size-[30px] "
        />
        <ActionsGroup icon="HeadBarPanelActive" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarSecondarySideBar" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarCustomizeLayout" className="size-[30px] " iconClassName="size-[18px]" /> */}
      </div>

      <div className="flex items-center">
        <ActionsGroup icon="HeadBarAccount" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarNotifications" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarWrench" className="size-[30px] " iconClassName="size-[18px]" />
      </div>
    </div>
  );
};
