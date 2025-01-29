import { ComponentPropsWithoutRef, forwardRef, useCallback } from "react";

import { ActivityBarState, useChangeActivityBarState, useGetActivityBarState } from "@/hooks/useActivityBarState";
import { useChangeProjectSessionState, useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useGetViewGroups } from "@/hooks/useViewGroups";
import { cn, Icon, Icons } from "@repo/moss-ui";

const positions = ["top", "bottom", "left", "right"] as const;

export const ActivityBar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { mutate: changeActivityBarState } = useChangeActivityBarState();

  const { data: viewGroups } = useGetViewGroups();

  const { data: projectSessionState } = useGetProjectSessionState();

  const { mutate: changeProjectSessionState } = useChangeProjectSessionState();

  const activityBarGroups = viewGroups?.viewGroups;

  const toggleActiveGroup = useCallback(
    (id: string) => {
      if (!projectSessionState) return;

      changeProjectSessionState({
        ...projectSessionState,
        lastActiveGroup: id,
      });
    },
    [changeProjectSessionState, projectSessionState]
  );

  const handleSelectPosition = (position: ActivityBarState["position"]) => {
    const index = positions.indexOf(position);
    if (index === 3) changeActivityBarState({ position: "top" });
    else changeActivityBarState({ position: positions[index + 1] });
  };

  if (activityBarState?.position === "top" || activityBarState?.position === "bottom") {
    return (
      <div
        className={cn("flex w-full items-center gap-2.5 border-solid bg-[#F4F4F4] px-2 py-1", {
          "border-b-[#c6c6c6]": activityBarState?.position === "top",
          "border-t-[#c6c6c6]": activityBarState?.position === "bottom",
        })}
        onDoubleClick={() => handleSelectPosition(activityBarState?.position)}
      >
        {activityBarGroups?.map(({ icon, id }, index) => (
          <ActivityBarButton
            key={index}
            icon={icon as Icons}
            active={projectSessionState?.lastActiveGroup === id}
            onClick={() => toggleActiveGroup(id)}
          />
        ))}
      </div>
    );
  }

  return (
    <div
      className={cn("flex h-full flex-col items-center gap-2.5 border bg-[#F4F4F4] px-1 py-2", {
        "border-r-[#c6c6c6]": activityBarState?.position === "left",
        "border-l-[#c6c6c6]": activityBarState?.position === "right",
      })}
      onDoubleClick={() => handleSelectPosition(activityBarState?.position as ActivityBarState["position"])}
    >
      {activityBarGroups?.map(({ icon, id }, index) => (
        <ActivityBarButton
          key={index}
          icon={icon as Icons}
          active={projectSessionState?.lastActiveGroup === id}
          onClick={() => toggleActiveGroup(id)}
        />
      ))}
    </div>
  );
};

interface ActivityBarButtonProps extends ComponentPropsWithoutRef<"div"> {
  icon: Icons;
  active?: boolean;
  iconClassName?: string;
}

const ActivityBarButton = forwardRef<HTMLDivElement, ActivityBarButtonProps>(
  ({ icon, active = false, iconClassName, ...props }, ref) => {
    return (
      <div
        ref={ref}
        {...props}
        className={cn("flex size-7 cursor-pointer items-center justify-center rounded-md", {
          "bg-[#D4E2FF]": active,
          "hover:bg-[#d3d3d1]": !active,
        })}
      >
        <Icon
          icon={icon}
          className={cn(
            {
              "text-(--moss-statusBar-background)": active,
              "text-[#525252]": !active,
            },
            iconClassName
          )}
        />
      </div>
    );
  }
);
