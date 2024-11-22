import { Icon, Icons } from "@repo/ui";
import type { ComponentPropsWithoutRef } from "react";
import { twMerge } from "tailwind-merge";

const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  return (
    <footer className={twMerge("flex h-[26px] w-screen justify-between bg-[#0F62FE] pr-[26px]", className)}>
      <div className="flex h-full">
        <StatusBarButton icon="StatusBarMacButton" className="bg-[#054ADA] px-[9px] py-[5px]" iconClassName="size-4" />

        <div className="flex h-full gap-1">
          <StatusBarButton icon="StatusBarTerminal" label="Terminal" />
          <StatusBarButton icon="StatusBarCommit" label="12 Commit" />
          <StatusBarButton icon="StatusBarSearch" label="Search" />
        </div>
      </div>

      <div className="flex h-full gap-1">
        <StatusBarButton icon="StatusBarGitlens" label="2 weeks ago, you" />
        <StatusBarButton label="UTF-8" />
        <StatusBarButton label="24 Ln, 16 Col" />
        <StatusBarButton label="4 Spaces" />
        <StatusBarButton label="Rust" />
        <StatusBarButton icon="StatusBarError" label="2 Errors" iconClassName="size-[6px]" />
        <StatusBarButton icon="StatusBarWarning" label="15 Warnings" iconClassName="size-[6px]" />
        <StatusBarButton label="--READ--" />
      </div>
    </footer>
  );
};

export default StatusBar;

interface StatusBarButtonProps extends ComponentPropsWithoutRef<"button"> {
  icon?: Icons;
  label?: string;
  className?: string;
  iconClassName?: string;
}

const StatusBarButton = ({ icon, iconClassName, label, className }: StatusBarButtonProps) => {
  return (
    <button
      className={twMerge(
        "group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-opacity-10 focus:bg-white focus:bg-opacity-10",
        className
      )}
    >
      {icon && <Icon className={twMerge(" size-[18px]", iconClassName)} icon={icon} />}
      {label && <span className="text-sm">{label}</span>}
    </button>
  );
};
