import { cn, DropdownMenu as DM, Icon, Icons } from "@repo/ui";
import { ComponentPropsWithoutRef } from "react";

interface ActionsGroupProps extends ComponentPropsWithoutRef<"div"> {
  icon: Icons;
  label?: string;
  compact?: boolean;
  //
  defaultAction?: boolean;
  actions: string[];
}

export const ActionsGroup = ({ compact = false, defaultAction = false, icon, label, ...props }: ActionsGroupProps) => {
  if (!defaultAction) {
    return (
      <div
        className={cn(
          "group box-border flex h-[30px] items-center rounded border border-transparent transition hover:border-[#c5c5c5]",
          props.className
        )}
        {...props}
      >
        <DM.Root>
          <DM.Trigger asChild>
            <button className="group flex h-full items-center gap-1.5 text-ellipsis rounded-l rounded-r px-2 hover:bg-[#D3D3D3]">
              <Icon icon={icon} className="group-active:text-black text-[#525252]" />
              {!compact && <span className="group-active:text-black text-ellipsis text-[#161616]">{label}</span>}
              <Icon icon="ArrowheadDown" />
            </button>
          </DM.Trigger>

          <DM.Content className="z-50">123</DM.Content>
        </DM.Root>
      </div>
    );
  }

  return (
    <div
      className={cn(
        "group relative box-border flex h-[30px] items-center rounded border border-transparent transition hover:border-[#c5c5c5]",
        props.className
      )}
      {...props}
    >
      <button className="group flex h-full items-center gap-1.5 text-ellipsis rounded-l px-2 hover:bg-[#D3D3D3]">
        <Icon icon={icon} className="group-active:text-black text-[#525252]" />
        {!compact && <span className="group-active:text-black text-ellipsis text-[#161616]">{label}</span>}
      </button>

      <div className="h-full w-px group-hover:bg-[#c5c5c5]" />

      <DM.Root>
        <DM.Trigger asChild>
          <button className="h-full rounded-r hover:bg-[#D3D3D3]">
            {props.actions.length > 1 && <Icon icon="ArrowheadDown" />}
          </button>
        </DM.Trigger>

        <DM.Content className="z-50">123</DM.Content>
      </DM.Root>
    </div>
  );
};
