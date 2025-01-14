import { useGetActivityBarState } from "@/hooks/useActivityBarState";
import { useGetViewGroup, useGetViewGroups } from "@/hooks/useViewGroups";

import SidebarHeader from "../parts/SideBar/SidebarHeader";
import { AccordionsList } from "./AccordionsList";
import { ActivityBar } from "./ActivityBar";

export const LaunchPad = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { data: viewGroups } = useGetViewGroups();

  const activeGroup = viewGroups?.find((group) => group.active);
  const activeGroupTitle = activeGroup?.title || "Launchpad";
  const activeGroupId = activeGroup?.id || "";

  if (activityBarState?.position === "top") {
    return (
      <div className="flex h-full flex-col background-[--moss-sideBar-background]">
        <ActivityBar />
        <SidebarHeader title={activeGroupTitle} />
        <LaunchpadContent groupId={activeGroupId} />
      </div>
    );
  }

  if (activityBarState?.position === "bottom") {
    return (
      <div className="flex h-full flex-col background-[--moss-sideBar-background]">
        <SidebarHeader title={activeGroupTitle} />
        <LaunchpadContent groupId={activeGroupId} />
        <ActivityBar />
      </div>
    );
  }

  if (activityBarState?.position === "left") {
    return (
      <div className="flex h-full background-[--moss-sideBar-background]">
        <ActivityBar />
        <div className="w-full">
          <SidebarHeader title={activeGroupTitle} />
          <LaunchpadContent groupId={activeGroupId} />
        </div>
      </div>
    );
  }

  if (activityBarState?.position === "right") {
    return (
      <div className="flex h-full background-[--moss-sideBar-background]">
        <div className="w-full">
          <SidebarHeader title={activeGroupTitle} />
          <LaunchpadContent groupId={activeGroupId} />
        </div>
        <ActivityBar />
      </div>
    );
  }

  return <div>Launchpad is empty</div>;
};

const LaunchpadContent = ({ groupId }: { groupId: string }) => {
  const { data: viewGroup } = useGetViewGroup(groupId);

  if (!viewGroup) return <div>Loading...</div>;

  if (viewGroup.id === "explorer") {
    return (
      <div className="h-full">
        <AccordionsList />
      </div>
    );
  }
  if (viewGroup.id === "activities") {
    return (
      <div className="h-full">
        <div>Activities</div>
      </div>
    );
  }

  return <div className="flex h-full flex-col">No group view was returned</div>;
};
