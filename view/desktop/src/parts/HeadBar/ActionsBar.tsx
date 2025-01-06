import { HTMLProps, useEffect, useState } from "react";

import { ActionButton } from "@/components/Action/ActionButton";
import { ActionsSubmenu } from "@/components/Action/ActionsSubmenu";
import { ActionsGroup } from "@/components/ActionsGroup";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useActivityBarStore } from "@/store/activityBar";
import { LayoutAlignment, LayoutPrimarySideBarPosition, useLayoutStore } from "@/store/layout";
import { MenuItem } from "@repo/moss-desktop";
import { cn, DropdownMenu } from "@repo/moss-ui";

export const ActionsBar = ({ className, ...props }: HTMLProps<HTMLDivElement>) => {
  const [activities, setActivities] = useState<MenuItem[]>([]);

  const {
    primarySideBar,
    secondarySideBar,
    bottomPane,
    setAlignment,
    alignment,
    primarySideBarPosition,
    setPrimarySideBarPosition,
  } = useLayoutStore((state) => state);

  const { position: activityBarPosition, setPosition: setActivityBarPosition } = useActivityBarStore();

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
      <table className="absolute left-0 top-0 bg-storm-800 p-2 text-white opacity-50">
        <tbody>
          <tr>
            <th>primarySideBarPosition</th>
            <th>{primarySideBarPosition}</th>
          </tr>
          <tr>
            <th>primarySideBar.width</th>
            <th>{primarySideBar.width}</th>
          </tr>
          <tr>
            <th>secondarySideBar.width</th>
            <th>{secondarySideBar.width}</th>
          </tr>
        </tbody>
      </table>

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

            const visibility =
              primarySideBarPosition === "left" ? primarySideBar.visibility : secondarySideBar.visibility;

            const handleVisibilityChange = () => {
              if (primarySideBarPosition === "left") {
                primarySideBar.setVisibility(!visibility);
              } else {
                secondarySideBar.setVisibility(!visibility);
              }
            };

            if (index === 0) {
              return (
                <ActionButton
                  key={command.id}
                  iconClassName="size-[18px]"
                  {...command}
                  icon={visibility ? "HeadBarPrimarySideBarActive" : "HeadBarPrimarySideBar"}
                  onClick={handleVisibilityChange}
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
              const visibility =
                primarySideBarPosition === "right" ? primarySideBar.visibility : secondarySideBar.visibility;

              const handleVisibilityChange = () => {
                if (primarySideBarPosition === "right") {
                  primarySideBar.setVisibility(!visibility);
                } else {
                  secondarySideBar.setVisibility(!visibility);
                }
              };
              return (
                <ActionButton
                  key={command.id}
                  iconClassName="size-[18px]"
                  {...command}
                  icon={visibility ? "HeadBarSecondarySideBarActive" : "HeadBarSecondarySideBar"}
                  onClick={handleVisibilityChange}
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
                <DropdownMenu.RadioGroup
                  value={alignment}
                  onValueChange={(value) => setAlignment(value as LayoutAlignment)}
                >
                  <DropdownMenu.RadioItem value="center" label="Center" checked={alignment === "center"} />
                  <DropdownMenu.RadioItem value="justify" label="Justify" checked={alignment === "justify"} />
                  <DropdownMenu.RadioItem value="left" label="Left" checked={alignment === "left"} />
                  <DropdownMenu.RadioItem value="right" label="Right" checked={alignment === "right"} />
                </DropdownMenu.RadioGroup>
                <DropdownMenu.Separator />
                <DropdownMenu.RadioGroup
                  value={primarySideBarPosition}
                  onValueChange={(value) => {
                    if (activityBarPosition === "left") {
                      setActivityBarPosition("right");
                    } else if (activityBarPosition === "right") {
                      setActivityBarPosition("left");
                    }

                    setPrimarySideBarPosition(value as LayoutPrimarySideBarPosition);
                  }}
                >
                  <DropdownMenu.RadioItem value="left" label="Left" checked={primarySideBarPosition === "left"} />
                  <DropdownMenu.RadioItem value="right" label="Right" checked={primarySideBarPosition === "right"} />
                </DropdownMenu.RadioGroup>
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
