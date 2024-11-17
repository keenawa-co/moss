import { cn, DropdownMenu as DM, Icon, Icons } from "@repo/ui";
import { ComponentPropsWithoutRef, useState } from "react";

interface ActionsGroupProps extends ComponentPropsWithoutRef<"div"> {
  icon: Icons;
  label?: string;
  compact?: boolean;
  iconClassName?: string;

  defaultAction?: boolean;
  actions?: string[];
}

const buttonStyle = "hover:border-[#c5c5c5] box-border transition group flex rounded border border-transparent";
const triggerStyle = "hover:bg-[#D3D3D3] group flex w-full items-center justify-center gap-1.5 text-ellipsis";
const iconStyle = "group-active:text-black text-[#525252]";
const labelStyle = "group-active:text-black text-ellipsis text-[#161616]";

export const ActionsGroup = ({
  compact = false,
  defaultAction = false,
  icon,
  label,
  className,
  iconClassName,
  ...props
}: ActionsGroupProps) => {
  const [open, setOpen] = useState(false);

  const showActions = props.actions !== undefined && props.actions.length > 1;

  if (!defaultAction) {
    return (
      <div className={cn(buttonStyle, className)} {...props}>
        <DM.Root open={open} onOpenChange={() => {}}>
          <DM.Trigger className={cn(triggerStyle, "rounded-r px-1.5 py-1.5")} onClick={() => setOpen((prev) => !prev)}>
            <Icon icon={icon} className={cn(iconStyle, iconClassName)} />
            {!compact && label && <span className={labelStyle}>{label}</span>}
            {showActions && <Icon icon="ArrowheadDown" className="ml-auto" />}
          </DM.Trigger>

          {showActions && (
            <DM.Content className="z-50 flex flex-col" onPointerDownOutside={() => setOpen(false)}>
              {props.actions?.map((id) => <button key={id}>Action {id}</button>)}
            </DM.Content>
          )}
        </DM.Root>
      </div>
    );
  }

  return (
    <div className={cn(buttonStyle, className)} {...props}>
      <div className="flex items-stretch">
        <button className={cn(triggerStyle, "px-1.5 py-1.5")}>
          <Icon icon={icon} className={cn(iconStyle, iconClassName)} />
          {!compact && label && <span className={labelStyle}>{label}</span>}
        </button>

        {showActions && (
          <>
            <div className="flex min-w-px grow self-stretch bg-transparent group-hover:bg-[#c5c5c5]" />
            <DM.Root open={open}>
              <DM.Trigger
                className={cn(triggerStyle, "self-stretch rounded-r")}
                onClick={() => setOpen((prev) => !prev)}
              >
                <Icon icon="ArrowheadDown" />
              </DM.Trigger>

              <DM.Content className="z-50 flex flex-col" onPointerDownOutside={() => setOpen(false)}>
                {props.actions?.map((id) => <button key={id}>Action {id}</button>)}
              </DM.Content>
            </DM.Root>
          </>
        )}
      </div>
    </div>
  );
};
