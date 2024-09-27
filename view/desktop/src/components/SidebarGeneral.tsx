import { Icon } from "@repo/ui";

export const SidebarGeneral = () => {
  return (
    <div className="cursor-pointer font-bold flex items-center gap-2 text-[14px]">
      <div>
        <Icon icon="Clock" className="size-[18px]" />
      </div>
      <span>Recents</span>
    </div>
  );
};
