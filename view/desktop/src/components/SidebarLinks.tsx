import { Icon } from "@repo/ui";

export const SidebarLinks = () => {
  return (
    <ul className="text-stone-500 text-[14px] flex flex-col gap-[14px] mt-[10px]">
      <li className="flex gap-[5px] items-center">
        <span>Moss Docs</span> <Icon icon="ArrowTopRight" className="size-[7px]" />
      </li>
      <li className="flex gap-[5px] items-center">
        <span>Moss Releases</span> <Icon icon="ArrowTopRight" className="size-[7px]" />
      </li>
      <li className="flex gap-[5px] items-center">
        <span>Moss GitHub</span> <Icon icon="ArrowTopRight" className="size-[7px]" />
      </li>
      <li className="flex gap-[5px] items-center">
        <span>Moss Support</span> <Icon icon="ArrowTopRight" className="size-[7px]" />
      </li>
    </ul>
  );
};
