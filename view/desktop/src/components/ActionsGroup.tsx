import { cn, DropdownMenu as DM, Icon, Icons } from "@repo/ui";
import { ComponentPropsWithoutRef, useState } from "react";

interface ActionsGroupProps extends ComponentPropsWithoutRef<"div"> {
  icon: Icons;
  label?: string;
  compact?: boolean;
  //
  defaultAction?: boolean;
  actions: string[];
}

export const ActionsGroup = ({ compact = false, defaultAction = false, icon, label, ...props }: ActionsGroupProps) => {
  const [open, setOpen] = useState(false);

  if (!defaultAction) {
    return (
      <div
        className={cn(
          "group box-border  flex h-[30px] items-center rounded border border-transparent transition hover:border-[#c5c5c5]",
          props.className
        )}
      >
        <DM.Root open={open} onOpenChange={() => {}}>
          <DM.Trigger asChild>
            <button
              className="DMTrigger group flex h-full w-full items-center gap-1.5 text-ellipsis rounded-l rounded-r px-2 hover:bg-[#D3D3D3]"
              onClick={() => setOpen((prev) => !prev)}
            >
              <Icon icon={icon} className="group-active:text-black text-[#525252]" />
              {!compact && <span className="group-active:text-black text-ellipsis text-[#161616]">{label}</span>}
              <Icon icon="ArrowheadDown" className="ml-auto" />
            </button>
          </DM.Trigger>

          <DM.Content className="z-50" onPointerDownOutside={() => setOpen(false)}>
            123
          </DM.Content>
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
    >
      <button className="group flex h-full w-full items-center gap-1.5 text-ellipsis rounded-l px-2 hover:bg-[#D3D3D3]">
        <Icon icon={icon} className="group-active:text-black text-[#525252]" />
        {!compact && <span className="group-active:text-black text-ellipsis text-[#161616]">{label}</span>}
      </button>

      {props.actions.length > 1 && (
        <>
          <div className="h-full w-px group-hover:bg-[#c5c5c5]" />
          <DM.Root open={open}>
            <DM.Trigger asChild>
              <button
                className="DMTrigger h-full rounded-r hover:bg-[#D3D3D3]"
                onClick={() => setOpen((prev) => !prev)}
              >
                <Icon icon="ArrowheadDown" />
              </button>
            </DM.Trigger>

            <DM.Content className="z-50" onPointerDownOutside={() => setOpen(false)}>
              123
            </DM.Content>
          </DM.Root>
        </>
      )}
    </div>
  );
};
