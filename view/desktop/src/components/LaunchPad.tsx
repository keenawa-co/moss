import { useGetActivityBarState } from "@/hooks/useActivityBarState";

import SidebarHeader from "../parts/SideBar/SidebarHeader";
import { AccordionsList } from "./AccordionsList";
import { ActivityBar } from "./ActivityBar";

export const LaunchPad = () => {
  const { data: activityBarState } = useGetActivityBarState();

  if (activityBarState?.position === "top") {
    return (
      <div className="flex h-full flex-col">
        <ActivityBar />
        <SidebarHeader title="launchpad" />
        <AccordionsList />
      </div>
    );
  }

  if (activityBarState?.position === "bottom") {
    return (
      <div className="flex h-full flex-col">
        <SidebarHeader title="launchpad" />
        <AccordionsList />
        <ActivityBar />
      </div>
    );
  }

  if (activityBarState?.position === "left") {
    return (
      <div className="flex h-full">
        <ActivityBar />
        <div className="w-full">
          <SidebarHeader title="launchpad" />
          <AccordionsList />
        </div>
      </div>
    );
  }

  if (activityBarState?.position === "right") {
    return (
      <div className="flex h-full">
        <div className="w-full">
          <SidebarHeader title="launchpad" />
          <AccordionsList />
        </div>
        <ActivityBar />
      </div>
    );
  }
};
