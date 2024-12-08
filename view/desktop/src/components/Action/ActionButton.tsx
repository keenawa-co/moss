import { CommandAction, MenuItemVisibility } from "@repo/desktop-models";
import { cn, Icon, Icons } from "@repo/moss-ui";
import { ComponentPropsWithoutRef } from "react";

interface ActionsGroupProps extends Omit<ComponentPropsWithoutRef<"div">, "id" | "title">, CommandAction {
  icon: Icons;
  iconClassName?: string;
  visibility: MenuItemVisibility;
}

const buttonStyle = "hover:border-[#c5c5c5] box-border transition group flex rounded border border-transparent";
const triggerStyle = "hover:bg-[#D3D3D3] group flex w-full items-center justify-center gap-1.5 text-ellipsis";
const iconStyle = "group-active:text-black text-[#525252]";
const labelStyle = "group-active:text-black text-[#161616] break-keep w-max";

export const ActionButton = ({ icon, className, iconClassName, title, visibility, ...props }: ActionsGroupProps) => {
  const [key, origin, description] = title as string[];

  return (
    <div className={cn(buttonStyle, className)} {...props}>
      <button className={cn(triggerStyle, "px-1.5 py-1")}>
        <Icon icon={icon} className={cn(iconStyle, iconClassName)} />
        {visibility === "classic" && origin && <span className={labelStyle}>{origin}</span>}
      </button>
    </div>
  );
};
