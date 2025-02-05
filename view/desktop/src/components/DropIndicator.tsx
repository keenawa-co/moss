import type { CSSProperties, HTMLAttributes } from "react";

import type { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/types";
import { cn } from "@repo/moss-ui";

type Orientation = "horizontal" | "vertical";

const edgeToOrientationMap: Record<Edge, Orientation> = {
  top: "horizontal",
  bottom: "horizontal",
  left: "vertical",
  right: "vertical",
};

const orientationStyles: Record<Orientation, HTMLAttributes<HTMLElement>["className"]> = {
  horizontal: "h-(--line-thickness) left-(--terminal-radius) right-0 before:left-(--negative-terminal-size)",
  vertical: "w-(--line-thickness) top-(--terminal-radius) bottom-0 before:top-(--negative-terminal-size)",
};

const edgeStyles: Record<Edge, HTMLAttributes<HTMLElement>["className"]> = {
  top: "top-(--line-offset) before:top-(--offset-terminal)",
  right: "right-(--line-offset) before:right-(--offset-terminal)",
  bottom: "bottom-(--line-offset) before:bottom-(--offset-terminal)",
  left: "left-(--line-offset) before:left-(--offset-terminal)",
};

/**
 * This is a tailwind port of `@atlaskit/pragmatic-drag-and-drop-react-drop-indicator/box`
 */
interface DropIndicatorProps {
  edge: Edge;
  gap?: number;
  strokeSize?: number;
  terminalSize?: number;
  className?: string;
}

export function DropIndicator({ edge, gap = 0, strokeSize = 2, terminalSize = 8, className }: DropIndicatorProps) {
  const lineOffset = -0.5 * (gap + strokeSize);
  const offsetToAlignTerminalWithLine = (strokeSize - terminalSize) / 2;

  const orientation = edgeToOrientationMap[edge];

  return (
    <div
      style={
        {
          "--line-thickness": `${strokeSize}px`,
          "--line-offset": `${lineOffset}px`,
          "--terminal-size": `${terminalSize}px`,
          "--terminal-radius": `${terminalSize / 2}px`,
          "--negative-terminal-size": `-${terminalSize}px`,
          "--offset-terminal": `${offsetToAlignTerminalWithLine}px`,
        } as CSSProperties
      }
      className={cn(
        `pointer-events-none absolute z-10 box-border bg-sky-700 before:absolute before:h-(--terminal-size) before:w-(--terminal-size) before:rounded-full before:border-[length:--line-thickness] before:border-solid before:border-sky-700 before:content-['']`,
        orientationStyles[orientation],
        edgeStyles[edge],
        className
      )}
    />
  );
}
