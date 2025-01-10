import { twMerge } from "tailwind-merge";

import { Icon, IconTitle, MenuItem } from "@repo/moss-ui";

enum IconState {
  Default = "group-text-[--moss-primary]",
  DefaultStroke = "group-stroke-zinc-500",
  Hover = "group-hover:text-[--moss-primary]",
  HoverStroke = "group-hover:stroke-zinc-600",
  Active = "text-[--moss-primary]",
  Disabled = "text-[--moss-primary] bg-opacity-50",
}

export const Sidebar = () => {
  return (
    <aside className="mb-5.5 flex w-full flex-col overflow-auto p-0 background-[--moss-sideBar-background]">
      <MenuItem className="bg-zinc-200 group mb-3.5 mt-13">
        <Icon icon="Search" className={twMerge("h-4.5 w-4.5 min-w-4", IconState.Default, IconState.Hover)} />
        <IconTitle className="text-xs text-[--moss-primary]" title={`"Search... ${Math.random().toFixed(3)}"`} />

        <Icon icon="SearchShortcut" className="fill-zinc-500 group-hover:fill-zinc-600 ml-auto w-4.5 min-w-4 pr-2" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Home1" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-sm text-[--moss-primary]" title="Home" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Issues" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-sm text-[--moss-primary]" title="Issues" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Code" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-sm text-[--moss-primary]" title="Code" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Goals" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-sm text-[--moss-primary]" title="Goals" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Reports" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-sm text-[--moss-primary]" title="Reports" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Documentation" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle
          className="text-sm text-[--moss-primary]"
          title="Documentation with very long title to trigger overflow X"
        />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Settings" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-sm text-[--moss-primary]" title="Settings" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="QuickSearch" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-sm text-[--moss-primary]" title="Quick Search" />
      </MenuItem>
    </aside>
  );
};

export default Sidebar;
