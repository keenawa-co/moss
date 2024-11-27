import { cn, Icon, Icons } from "../../../../packages/moss-ui/src";
import type { ComponentPropsWithoutRef } from "react";

const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  return (
    <footer className={cn("flex h-[26px] w-screen justify-between bg-[#0F62FE] pr-[26px]", className)}>
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

        <div className="group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-opacity-10 focus:bg-white focus:bg-opacity-10">
          <StatusCircle className="size-[6px] bg-[#D62A18]" />
          <span>2 Errors</span>
        </div>

        <div className="group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-opacity-10 focus:bg-white focus:bg-opacity-10">
          <StatusCircle className="size-[6px] bg-[#FFC505]" />
          <span>15 Warnings</span>
        </div>

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

const StatusCircle = ({ className }: { className?: string }) => {
  return <div className={cn("flex items-center justify-center rounded-full", className)} />;
};

const StatusBarButton = ({ icon, iconClassName, label, className }: StatusBarButtonProps) => {
  return (
    <button
      className={cn(
        "group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-opacity-10 focus:bg-white focus:bg-opacity-10",
        className
      )}
    >
      {icon && <Icon className={cn(" size-[18px]", iconClassName)} icon={icon} />}
      {label && <span className="text-sm">{label}</span>}
    </button>
  );
};
