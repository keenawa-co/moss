import { Icon } from "@repo/ui";

export const SidebarHeader = ({ title }: { title: string }) => {
  return (
    <div className="uppercase text-stone-500 font-semibold text-[14px] py-[10px] px-[15px] flex items-center justify-between">
      <span>{title}</span>
      <button className="hover:bg-stone-100 rounded p-1">
        <Icon icon="ThreeHorizontalDots" />
      </button>
    </div>
  );
};

export default SidebarHeader;
