import { Icon, IconTitle, MenuItem } from "@repo/ui";
import { SidebarLayout } from "@/components";
import { twMerge } from "tailwind-merge";

enum IconState {
  Default = "group-text-[rgba(var(--color-primary))]",
  DefaultStroke = "group-stroke-zinc-500",
  Hover = "group-hover:text-[rgba(var(--color-primary))]",
  HoverStroke = "group-hover:stroke-zinc-600",
  Active = "text-[rgba(var(--color-primary))]",
  Disabled = "text-[rgba(var(--color-primary))] bg-opacity-50",
}

export const Sidebar = () => {
  return (
    <SidebarLayout className="p-0 h-full w-full overflow-auto">
      <MenuItem className="group bg-zinc-200 mt-13 mb-3.5">
        <Icon icon="Search" className={twMerge("h-4.5 w-4.5 min-w-4", IconState.Default, IconState.Hover)} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-xs" title="Search..." />

        <Icon icon="SearchShortcut" className="min-w-4  w-4.5 fill-zinc-500 group-hover:fill-zinc-600  ml-auto pr-2" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Home1" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-sm" title="Home" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Issues" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-sm" title="Issues" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Code" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-sm" title="Code" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Goals" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-sm" title="Goals" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Reports" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-sm" title="Reports" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Documentation" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle
          className="text-[rgba(var(--color-primary))] text-sm"
          title="Documentation with very long title to trigger overflow X"
        />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="Settings" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-sm" title="Settings" />
      </MenuItem>

      <MenuItem className="group">
        <Icon icon="QuickSearch" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
        <IconTitle className="text-[rgba(var(--color-primary))] text-sm" title="Quick Search" />
      </MenuItem>
    </SidebarLayout>
  );
};
export default Sidebar;
