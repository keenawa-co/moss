import { useActivityBarStore } from "@/store/activityBar";

import SidebarHeader from "../parts/SideBar/SidebarHeader";
import { AccordionsList } from "./AccordionsList";
import { ActivityBar } from "./ActivityBar";

export const LaunchPad = () => {
  const { position } = useActivityBarStore();

  if (position === "top") {
    return (
      <div className="flex h-full flex-col">
        <ActivityBar />
        <SidebarHeader title="launchpad" />
        <AccordionsList />
      </div>
    );
  }
  if (position === "bottom") {
    return (
      <div className="flex h-full flex-col">
        <SidebarHeader title="launchpad" />
        <AccordionsList />
        <ActivityBar />
      </div>
    );
  }
  if (position === "left") {
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
  if (position === "right") {
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
