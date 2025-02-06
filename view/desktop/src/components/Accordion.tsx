import { ComponentPropsWithoutRef } from "react";

import { cn } from "@/utils";

import Icon from "./Icon";

interface AccordionProps extends ComponentPropsWithoutRef<"div"> {
  title: string;
  isOpen?: boolean;
  handleClick: () => void;
  children: React.ReactNode[] | React.ReactNode;
}

const Accordion = ({ title, isOpen = false, handleClick, children, ...props }: AccordionProps) => {
  return (
    <div className={cn(`h-full`, props.className)} {...props}>
      <div onClick={handleClick} className="flex cursor-pointer items-center px-2 py-[5px]">
        <div className={cn(`flex size-5 items-center justify-center`, { "rotate-90": isOpen })}>
          <Icon icon="ArrowheadRight" className="text-xs" />
        </div>
        <span className="font-bold">{title}</span>
      </div>

      <div className={isOpen ? "h-full overflow-auto pl-6 text-xs text-gray-500" : "visually-hidden"}>{children}</div>
    </div>
  );
};

export default Accordion;
