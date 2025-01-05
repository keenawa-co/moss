import { ComponentPropsWithoutRef, forwardRef, useState } from "react";

import { ActivityBarStore, useActivityBarStore } from "@/store/activityBar";
import { cn, Icon, Icons } from "@repo/moss-ui";

const positions = ["top", "bottom", "left", "right"] as const;

export const ActivityBar = () => {
  const { position, setPosition } = useActivityBarStore();
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

  const handleSelectPosition = (position: ActivityBarStore["position"]) => {
    const index = positions.indexOf(position);
    if (index === 3) setPosition("top");
    else setPosition(positions[index + 1]);
  };

  if (position === "top" || position === "bottom") {
    return (
      <div className="flex h-9 w-full items-center gap-2.5 px-2" onDoubleClick={() => handleSelectPosition(position)}>
        {list.map(({ icon, active }, index) => (
          <ActivityBarButton key={index} icon={icon as Icons} active={active} onClick={() => toggleActiveItem(index)} />
        ))}
      </div>
    );
  }

  return (
    <div
      className="flex h-full flex-col items-center gap-2.5 px-2"
      onDoubleClick={() => handleSelectPosition(position)}
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
