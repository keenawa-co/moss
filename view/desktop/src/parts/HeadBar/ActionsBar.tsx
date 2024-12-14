import { HTMLProps, useEffect, useState } from "react";

import { ActionButton } from "@/components/Action/ActionButton";
import { ActionsSubmenu } from "@/components/Action/ActionsSubmenu";
import { ActionsGroup } from "@/components/ActionsGroup";
import { invokeIpc } from "@/lib/backend/tauri";
import { useLayoutStore } from "@/store/layout";
import { MenuItem } from "@repo/desktop-models";
import { cn } from "@repo/moss-ui";

export const ActionsBar = ({ className, ...props }: HTMLProps<HTMLDivElement>) => {
  const [activities, setActivities] = useState<MenuItem[]>([]);

  const { primarySideBar, secondarySideBar, bottomPane, setAlignment, alignment } = useLayoutStore((state) => state);

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
          const icons = [
            "HeadBarPrimarySideBar",
            "HeadBarPanelActive",
            "HeadBarSecondarySideBar",
            "HeadBarCustomizeLayout",
          ] as const;

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
            // isTerminalVisible, setIsTerminalVisible
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
              >
                <ul className="flex flex-col gap-1">
                  <li
                    className={cn("cursor-pointer px-2 hover:bg-green-100", { "bg-green-300": alignment === "center" })}
                    onClick={() => setAlignment("center")}
                  >
                    Center
                  </li>
                  <li
                    className={cn("cursor-pointer px-2 hover:bg-green-100", {
                      "bg-green-300": alignment === "justify",
                    })}
                    onClick={() => setAlignment("justify")}
                  >
                    Justify
                  </li>
                  <li
                    className={cn("cursor-pointer px-2 hover:bg-green-100", { "bg-green-300": alignment === "left" })}
                    onClick={() => setAlignment("left")}
                  >
                    Left
                  </li>
                  <li
                    className={cn("cursor-pointer px-2 hover:bg-green-100", { "bg-green-300": alignment === "right" })}
                    onClick={() => setAlignment("right")}
                  >
                    Right
                  </li>
                </ul>
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
