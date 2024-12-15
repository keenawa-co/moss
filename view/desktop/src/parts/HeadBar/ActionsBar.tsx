import { HTMLProps, useEffect, useState } from "react";

import { ActionButton } from "@/components/Action/ActionButton";
import { ActionsSubmenu } from "@/components/Action/ActionsSubmenu";
import { ActionsGroup } from "@/components/ActionsGroup";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useLayoutStore } from "@/store/layout";
import { MenuItem } from "@repo/moss-desktop";
import { cn, DropdownMenu as DM } from "@repo/moss-ui";

export const ActionsBar = ({ className, ...props }: HTMLProps<HTMLDivElement>) => {
  const [activities, setActivities] = useState<MenuItem[]>([]);

  const { primarySideBar, secondarySideBar, bottomPane, setAlignment, alignment } = useLayoutStore((state) => state);

  useEffect(() => {
    const getAllActivities = async () => {
      try {
        const res = await invokeTauriIpc<MenuItem[], Error>("get_menu_items_by_namespace", { namespace: "headItem" }); // this here should be a type

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
        <ActionsSubmenu
          visibility="classic"
          defaultActionId={"qwe"}
          title={["main", "main", "#50"]}
          icon="HeadBarBranch"
          submenuId="-1"
          group={{ id: "main", order: BigInt(0), description: ["desc", "desc", "desc"] }}
          order={BigInt(0)}
          when=""
        />
      </div>

      <div className="flex items-center">
        {activities.map((item, index) => {
          if ("action" in item) {
            const command = item.action.command;

            if (index === 0) {
              return (
                <ActionButton
                  key={command.id}
                  iconClassName="size-[18px]"
                  {...command}
                  icon={primarySideBar.visibility ? "HeadBarPrimarySideBarActive" : "HeadBarPrimarySideBar"}
                  onClick={() => primarySideBar.setVisibility(!primarySideBar.visibility)}
                  visibility={item.action.visibility}
                />
              );
            }

            if (index === 1) {
              return (
                <ActionButton
                  key={command.id}
                  iconClassName="size-[18px]"
                  {...command}
                  icon={bottomPane.visibility ? "HeadBarPanelActive" : "HeadBarPanel"}
                  onClick={() => bottomPane.setVisibility(!bottomPane.visibility)}
                  visibility={item.action.visibility}
                />
              );
            }
            if (index === 2) {
              return (
                <ActionButton
                  key={command.id}
                  iconClassName="size-[18px]"
                  {...command}
                  icon={secondarySideBar.visibility ? "HeadBarSecondarySideBarActive" : "HeadBarSecondarySideBar"}
                  onClick={() => secondarySideBar.setVisibility(!secondarySideBar.visibility)}
                  visibility={item.action.visibility}
                />
              );
            }
          }

          if ("submenu" in item) {
            return (
              <ActionsSubmenu
                key={item.submenu.submenuId}
                iconClassName="size-[18px]"
                {...item.submenu}
                icon="HeadBarCustomizeLayout"
              >
                <DM.RadioGroup
                  value={alignment}
                  onValueChange={(value) => setAlignment(value as "center" | "justify" | "left" | "right")}
                >
                  <DM.RadioItem value="center" label="Center" checked={alignment === "center"} />
                  <DM.RadioItem value="justify" label="Justify" checked={alignment === "justify"} />
                  <DM.RadioItem value="left" label="Left" checked={alignment === "left"} />
                  <DM.RadioItem value="right" label="Right" checked={alignment === "right"} />
                </DM.RadioGroup>
              </ActionsSubmenu>
            );
          }
        })}
      </div>

      <div className="flex items-center">
        <ActionsGroup icon="HeadBarAccount" className="size-[30px]" iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarNotifications" className="size-[30px]" iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarWrench" className="size-[30px]" iconClassName="size-[18px]" />
      </div>
    </div>
  );
};
