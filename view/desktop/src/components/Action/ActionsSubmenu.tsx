import { MenuItemVisibility, SubmenuMenuItem } from "@repo/desktop-models";

import { cn, Icon, Icons, DropdownMenu as DM } from "@repo/ui";
import { ComponentPropsWithoutRef, useState } from "react";

const buttonStyle =
  "hover:border-[#c5c5c5] hover:bg-[#D3D3D3] box-border transition group flex rounded border border-transparent";
const triggerStyle = "hover:bg-[#D3D3D3] group flex w-full items-center justify-center gap-1.5 text-ellipsis";
const iconStyle = "group-active:text-black text-[#525252]";
const labelStyle = "group-active:text-black text-[#161616] break-keep w-max";

interface ActionsSubmenuProps extends Omit<ComponentPropsWithoutRef<"div">, "id" | "title">, SubmenuMenuItem {
  icon: Icons;
  iconClassName?: string;
  visibility: MenuItemVisibility;
}

export const ActionsSubmenu = ({
  icon,
  className,
  iconClassName,
  visibility,
  title,
  ...props
}: ActionsSubmenuProps) => {
  const [open, setOpen] = useState(false);
  const [key, origin, description] = title as string[];

  return (
    <div
      className={cn(buttonStyle, className, {
        "border-[#c5c5c5] bg-[#D3D3D3]": open,
      })}
      {...props}
    >
      <DM.Root open={open}>
        <DM.Trigger
          className={cn(triggerStyle, "self-stretch  px-1 py-1", {
            "bg-[#D3D3D3]": open,
          })}
          onClick={() => setOpen((prev) => !prev)}
        >
          <Icon icon={icon} className={cn(iconStyle, iconClassName)} />
          {visibility === "classic" && origin && <span className={labelStyle}>{origin}</span>}
          <Icon icon="ArrowheadDown" className="-mr-1" />
        </DM.Trigger>

        <DM.Content className="z-50 flex flex-col" onPointerDownOutside={() => setOpen(false)}>
          {[1, 2, 3]?.map((id) => <button key={id}>Action {id}</button>)}
        </DM.Content>
      </DM.Root>
    </div>
  );
};
