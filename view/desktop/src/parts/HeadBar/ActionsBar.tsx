import { HTMLProps } from "react";

import { DropdownMenu } from "@/components";
import { ActionButton } from "@/components/Action/ActionButton";
import { ActionsSubmenu } from "@/components/Action/ActionsSubmenu";
import { ActionsGroup } from "@/components/ActionsGroup";
import { useGetActivitiesState } from "@/hooks/useActivitiesState";
import { useChangeActivityBarState, useGetActivityBarState } from "@/hooks/useActivityBarState";
import { AppLayoutState, useChangeAppLayoutState, useGetAppLayoutState } from "@/hooks/useAppLayoutState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

export const ActionsBar = ({ className, ...props }: HTMLProps<HTMLDivElement>) => {
  const { data: activityBarState } = useGetActivityBarState();
  const { mutate: changeActivityBarState } = useChangeActivityBarState();

  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();

  const { data: activities } = useGetActivitiesState();

  const { primarySideBar, secondarySideBar, bottomPane } = useAppResizableLayoutStore((state) => state);

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
        {activities?.map((item, index) => {
          if ("action" in item) {
            const command = item.action.command;

            if (index === 0) {
              const visibility =
                appLayoutState?.primarySideBarPosition === "left"
                  ? primarySideBar.visibility
                  : secondarySideBar.visibility;

              const handleVisibilityChange = () => {
                if (appLayoutState?.primarySideBarPosition === "left") {
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
                appLayoutState?.primarySideBarPosition === "right"
                  ? primarySideBar.visibility
                  : secondarySideBar.visibility;

              const handleVisibilityChange = () => {
                if (appLayoutState?.primarySideBarPosition === "right") {
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
                  value={appLayoutState?.alignment}
                  onValueChange={(value) => {
                    changeAppLayoutState({
                      ...(appLayoutState as AppLayoutState),
                      alignment: value as AppLayoutState["alignment"],
                    });
                  }}
                >
                  <DropdownMenu.RadioItem
                    value="center"
                    label="Center"
                    checked={appLayoutState?.alignment === "center"}
                  />
                  <DropdownMenu.RadioItem
                    value="justify"
                    label="Justify"
                    checked={appLayoutState?.alignment === "justify"}
                  />
                  <DropdownMenu.RadioItem value="left" label="Left" checked={appLayoutState?.alignment === "left"} />
                  <DropdownMenu.RadioItem value="right" label="Right" checked={appLayoutState?.alignment === "right"} />
                </DropdownMenu.RadioGroup>
                <DropdownMenu.Separator />
                <DropdownMenu.RadioGroup
                  value={appLayoutState?.primarySideBarPosition}
                  onValueChange={(value) => {
                    if (activityBarState?.position === "left") {
                      changeActivityBarState({ position: "right" });
                    } else if (activityBarState?.position === "right") {
                      changeActivityBarState({ position: "left" });
                    }

                    changeAppLayoutState({
                      ...(appLayoutState as AppLayoutState),
                      primarySideBarPosition: value as AppLayoutState["primarySideBarPosition"],
                    });
                  }}
                >
                  <DropdownMenu.RadioItem
                    value="left"
                    label="Left"
                    checked={appLayoutState?.primarySideBarPosition === "left"}
                  />
                  <DropdownMenu.RadioItem
                    value="right"
                    label="Right"
                    checked={appLayoutState?.primarySideBarPosition === "right"}
                  />
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
