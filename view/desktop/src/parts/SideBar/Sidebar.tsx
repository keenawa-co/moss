import { ActivityBar } from "@/components/ActivityBar";
import { ViewContainer } from "@/components/ViewContainer";
import { useGetActivityBarState } from "@/hooks/useActivityBarState";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useGetViewGroups } from "@/hooks/useViewGroups";

import SidebarHeader from "./SidebarHeader";

export const Sidebar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { data: viewGroups } = useGetViewGroups();
  const { data: projectSessionState } = useGetProjectSessionState();

  const activeGroup = viewGroups?.viewGroups?.find((group) => group.id === projectSessionState?.lastActiveGroup);
  const activeGroupTitle = activeGroup?.title || "Launchpad";
  const activeGroupId = activeGroup?.id || "";

  if (activityBarState?.position === "top") {
    return (
      <div className="background-(--moss-sideBar-background) flex h-full flex-col">
        <ActivityBar />
        <SidebarHeader title={activeGroupTitle} />
        <ViewContainer groupId={activeGroupId} />
      </div>
    );
  }

  if (activityBarState?.position === "bottom") {
    return (
      <div className="background-(--moss-sideBar-background) flex h-full flex-col">
        <SidebarHeader title={activeGroupTitle} />
        <ViewContainer groupId={activeGroupId} />
        <ActivityBar />
      </div>
    );
  }

  if (activityBarState?.position === "left") {
    return (
      <div className="background-(--moss-sideBar-background) flex h-full">
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
      <div className="background-(--moss-sideBar-background) flex h-full">
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

export default Sidebar;
