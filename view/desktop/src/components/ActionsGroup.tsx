import { ComponentPropsWithoutRef, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";
import { cn, DropdownMenu as DM, Icon, Icons } from "@repo/moss-ui";

import { DropIndicator } from "./DropIndicator";

interface ActionsGroupProps extends Omit<ComponentPropsWithoutRef<"div">, "id"> {
  icon: Icons;
  label?: string;
  compact?: boolean;
  iconClassName?: string;
  defaultAction?: boolean;
  actions?: string[];

  id?: number;
  isDraggable?: boolean;
  draggableType?: string;
}

const buttonStyle =
  "relative hover:border-[#c5c5c5] box-border transition group flex rounded border border-transparent";
const triggerStyle = "hover:bg-[#D3D3D3] group flex w-full items-center justify-center gap-1.5 text-ellipsis";
const iconStyle = "group-active:text-black text-[#525252]";
const labelStyle = "group-active:text-black text-[#161616] break-keep w-max";

export const ActionsGroup = ({
  compact = false,
  defaultAction = false,
  icon,
  label,
  className,
  iconClassName,
  id,
  isDraggable,
  draggableType,
  ...props
}: ActionsGroupProps) => {
  const [open, setOpen] = useState(false);

  const showActions = props.actions !== undefined && props.actions.length > 1;
  const ref = useRef<HTMLDivElement | null>(null);
  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const dropIndicatorGap = 2;

  useEffect(() => {
    const element = ref.current;

    if (!element || !isDraggable) return;

    return combine(
      draggable({
        element: element,
        getInitialData: () => ({ id, icon, label }),
        onDrop: () => {
          setPreview(null);
        },
        onGenerateDragPreview({ nativeSetDragImage }) {
          setCustomNativeDragPreview({
            nativeSetDragImage,
            render({ container }) {
              setPreview((prev) => (prev === container ? prev : container));
            },
          });
        },
      }),
      dropTargetForElements({
        element,
        onDrop: () => {
          setClosestEdge(null);
        },
        getData({ input }) {
          return attachClosestEdge(
            { id, label, icon, draggableType },
            {
              element,
              input,
              allowedEdges: ["right", "left"],
            }
          );
        },
        getIsSticky() {
          return true;
        },
        onDragEnter({ self }) {
          const closestEdge = extractClosestEdge(self.data);
          setClosestEdge(closestEdge);
        },
        onDrag({ self }) {
          const closestEdge = extractClosestEdge(self.data);

          setClosestEdge((current) => {
            if (current === closestEdge) return current;

            return closestEdge;
          });
        },
        onDragLeave() {
          setClosestEdge(null);
        },
      })
    );
  }, [id, label, isDraggable, icon, draggableType]);

  if (!defaultAction) {
    return (
      <div
        ref={ref}
        className={cn(buttonStyle, className, {
          "border-[#c5c5c5]": open,
        })}
        {...props}
      >
        <DM.Root open={open} onOpenChange={() => {}}>
          <DM.Trigger
            className={cn(triggerStyle, "rounded-r px-1.5 py-1", {
              "bg-[#D3D3D3]": open,
            })}
            onClick={() => {
              if (showActions) setOpen((prev) => !prev);
            }}
          >
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
        {closestEdge ? <DropIndicator edge={closestEdge} gap={dropIndicatorGap} /> : null}
        {preview && createPortal(<ActionsGroup icon={icon} label={label} className="bg-sky-500" />, preview)}
      </div>
    );
  }

  return (
    <div
      ref={ref}
      className={cn(buttonStyle, className, {
        "border-[#c5c5c5]": open,
      })}
      {...props}
    >
      <div className="flex items-stretch">
        <button className={cn(triggerStyle, "px-1.5 py-1")}>
          <Icon icon={icon} className={cn(iconStyle, iconClassName)} />
          {!compact && label && <span className={labelStyle}>{label}</span>}
        </button>

        {showActions && (
          <>
            <div
              className={cn("flex min-w-px grow self-stretch bg-transparent group-hover:bg-[#c5c5c5]", {
                "bg-[#c5c5c5]": open,
              })}
            />
            <DM.Root open={open}>
              <DM.Trigger
                className={cn(triggerStyle, "self-stretch rounded-r", {
                  "bg-[#D3D3D3]": open,
                })}
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
      {closestEdge ? <DropIndicator edge={closestEdge} gap={dropIndicatorGap} /> : null}
      {preview && createPortal(<ActionsGroup icon={icon} label={label} />, preview)}
    </div>
  );
};
