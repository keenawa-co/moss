import { ComponentPropsWithoutRef, forwardRef, useState } from "react";

import { ActivityBarState, useChangeActivityBarState, useGetActivityBarState } from "@/hooks/useActivityBarState";
import { cn, Icon, Icons } from "@repo/moss-ui";

const positions = ["top", "bottom", "left", "right"] as const;

export const ActivityBar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { mutate: changeActivityBarState } = useChangeActivityBarState();

  const [list, setList] = useState([
    {
      icon: "ActivityBarIcon1",
      active: true,
    },
    {
      icon: "ActivityBarIcon2",
      active: false,
    },
  ]);

  const toggleActiveItem = (index: number) => {
    setList((prev) =>
      prev.map((item, i) => ({
        ...item,
        active: i === index,
      }))
    );
  };

  const handleSelectPosition = (position: ActivityBarState["position"]) => {
    const index = positions.indexOf(position);
    if (index === 3) changeActivityBarState({ position: "top" });
    else changeActivityBarState({ position: positions[index + 1] });
  };

  if (activityBarState?.position === "top" || activityBarState?.position === "bottom") {
    return (
      <div
        className={cn("flex w-full items-center gap-2.5 border bg-[#F4F4F4] px-2 py-1", {
          "border-b-[#c6c6c6]": activityBarState?.position === "top",
          "border-t-[#c6c6c6]": activityBarState?.position === "bottom",
        })}
        onDoubleClick={() => handleSelectPosition(activityBarState?.position)}
      >
        {list.map(({ icon, active }, index) => (
          <ActivityBarButton key={index} icon={icon as Icons} active={active} onClick={() => toggleActiveItem(index)} />
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
      {list.map(({ icon, active }, index) => (
        <ActivityBarButton key={index} icon={icon as Icons} active={active} onClick={() => toggleActiveItem(index)} />
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
              "text-[--color-statusBar-background]": active,
              "text-[#525252]": !active,
            },
            iconClassName
          )}
        />
      </div>
    );
  }
);
