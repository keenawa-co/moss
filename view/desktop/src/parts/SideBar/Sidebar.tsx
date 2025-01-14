import { ActivityBar } from "@/components/ActivityBar";
import { ViewContainer } from "@/components/ViewContainer";
import { useGetActivityBarState } from "@/hooks/useActivityBarState";
import { useGetViewGroups } from "@/hooks/useViewGroups";

import SidebarHeader from "./SidebarHeader";

export const Sidebar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { data: viewGroups } = useGetViewGroups();

  const activeGroup = viewGroups?.viewGroups?.find((group) => group.active);
  const activeGroupTitle = activeGroup?.title || "Launchpad";
  const activeGroupId = activeGroup?.id || "";

  if (activityBarState?.position === "top") {
    return (
      <div className="flex h-full flex-col background-[--moss-sideBar-background]">
        <ActivityBar />
        <SidebarHeader title={activeGroupTitle} />
        <ViewContainer groupId={activeGroupId} />
      </div>
    );
  }

  if (activityBarState?.position === "bottom") {
    return (
      <div className="flex h-full flex-col background-[--moss-sideBar-background]">
        <SidebarHeader title={activeGroupTitle} />
        <ViewContainer groupId={activeGroupId} />
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
          <ViewContainer groupId={activeGroupId} />
        </div>
      </div>
    );
  }

  if (activityBarState?.position === "right") {
    return (
      <div className="flex h-full background-[--moss-sideBar-background]">
        <div className="w-full">
          <SidebarHeader title={activeGroupTitle} />
          <ViewContainer groupId={activeGroupId} />
        </div>
        <ActivityBar />
      </div>
    );
  }

  return <div>Launchpad is empty</div>;
};
// mb-5.5 flex w-full flex-col overflow-auto p-0  background-[--moss-sideBar-background]

export default Sidebar;
