import { Icon, Icons } from "@repo/ui";
import { ComponentPropsWithoutRef } from "react";

interface HeadBarButtonProps extends ComponentPropsWithoutRef<"button"> {
  icon: Icons;
  label?: string;
}

export const HeadBarButton = ({ icon, label, ...props }: HeadBarButtonProps) => {
  return (
    <button className="group flex items-center gap-1.5 transition-colors " {...props}>
      <Icon icon={icon} className="text-[#525252] group-hover:text-[#0065FF] group-active:text-[#0747A6]" />
      {label && <span className="text-[#161616] group-hover:text-[#0065FF] group-active:text-[#0747A6]">{label}</span>}
    </button>
  );
};
