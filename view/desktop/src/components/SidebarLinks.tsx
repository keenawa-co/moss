import { Icon } from "@repo/ui";

export const SidebarLinks = () => {
  return (
    <ul className="mt-[10px] flex flex-col gap-[14px] text-[14px] text-stone-500">
      <li className="flex items-center gap-[5px]">
        <span>Moss Docs</span> <Icon icon="ArrowTopRight" className="mt-[2px] size-[7px]" />
      </li>
      <li className="flex items-center gap-[5px]">
        <span>Moss Releases</span> <Icon icon="ArrowTopRight" className="mt-[2px] size-[7px]" />
      </li>
      <li className="flex items-center gap-[5px]">
        <span>Moss GitHub</span> <Icon icon="ArrowTopRight" className="mt-[2px] size-[7px]" />
      </li>
      <li className="flex items-center gap-[5px]">
        <span>Moss Support</span> <Icon icon="ArrowTopRight" className="mt-[2px] size-[7px]" />
      </li>
    </ul>
  );
};
