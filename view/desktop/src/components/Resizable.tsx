import * as ResizablePrimitive from "react-resizable-panels";

import { cn } from "../utils/index";
import React, { useRef } from "react";

const ResizablePanelGroup = React.forwardRef<
  React.ElementRef<typeof ResizablePrimitive.PanelGroup>,
  React.ComponentPropsWithoutRef<typeof ResizablePrimitive.PanelGroup>
>(({ className, ...props }, ref) => (
  <ResizablePrimitive.PanelGroup
    ref={ref}
    className={cn(`flex h-full w-full data-[panel-group-direction=vertical]:flex-col`, className)}
    {...props}
  />
));

const ResizablePanel = ResizablePrimitive.Panel;

const ResizableHandle = React.forwardRef<
  React.ElementRef<typeof ResizablePrimitive.PanelResizeHandle>,
  React.ComponentPropsWithoutRef<typeof ResizablePrimitive.PanelResizeHandle> & {
    withHandle?: boolean;
  }
>(({ withHandle, className, ...props }, ref) => {
  return (
    <ResizablePrimitive.PanelResizeHandle
      ref={ref}
      className={cn(
        `relative flex w-px items-center justify-center bg-border

        after:absolute 
        after:inset-y-0 
        after:left-1/2 
        after:w-1 
        after:-translate-x-1/2 

        focus-visible:outline-none 
        focus-visible:ring-1 
        focus-visible:ring-ring 
        focus-visible:ring-offset-1 

        data-[panel-group-direction=vertical]:h-px 
        data-[panel-group-direction=vertical]:w-full 
        data-[panel-group-direction=vertical]:after:left-0 
        data-[panel-group-direction=vertical]:after:h-1 
        data-[panel-group-direction=vertical]:after:w-full 
        data-[panel-group-direction=vertical]:after:-translate-y-1/2 
        data-[panel-group-direction=vertical]:after:translate-x-0 
        [&[data-panel-group-direction=vertical]>div]:rotate-90

      data-[resize-handle-state=hover]:bg-blue-400
      
      data-[panel-group-direction=vertical][data-resize-handle-state=drag]:w-full 
      data-[panel-group-direction=vertical][data-resize-handle-state=drag]:h-[3px] 

      data-[panel-group-direction=horizontal][data-resize-handle-state=drag]:w-[3px] 
      data-[panel-group-direction=horizontal][data-resize-handle-state=drag]:bg-blue-400
    `,
        className
      )}
      {...props}
    >
      {withHandle && (
        <div className={cn(`z-10 flex h-4 w-3 items-center justify-center rounded-sm border bg-border`)}>
          <div className="h-6 w-1 bg-blue-400" />
        </div>
      )}
    </ResizablePrimitive.PanelResizeHandle>
  );
});

export { ResizablePanelGroup, ResizablePanel, ResizableHandle };
