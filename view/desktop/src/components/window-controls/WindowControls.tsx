import { useEffect, useState } from "react";
import { cn } from "@/components/window-controls/libs/utils";
import { TauriAppWindowProvider } from "@/components/window-controls/contexts";
import { Gnome, MacOS, Windows } from "@/components/window-controls/controls";
import { getOsType } from "@/components/window-controls/libs/plugin-os";
import type { WindowControlsProps } from "@/components/window-controls/types";

export function WindowControls({
  platform,
  justify = false,
  hide = false,
  hideMethod = "display",
  className,
  ...props
}: WindowControlsProps) {
  const [osType, setOsType] = useState<string | undefined>(undefined);

  // for macOS testing: setOsType("macos");
  useEffect(() => {
    getOsType().then((type) => {
      setOsType(type);
    });
  }, []);

  const customClass = cn("flex", className, hide && (hideMethod === "display" ? "hidden" : "invisible"));

  // Determine the default platform based on the operating system if not specified
  if (!platform) {
    switch (osType) {
      case "macos":
        platform = "macos";
        break;
      case "linux":
        platform = "gnome";
        break;
      default:
        platform = "windows";
    }
  }

  const ControlsComponent = () => {
    switch (platform) {
      case "windows":
        return <Windows className={cn(customClass, justify && "ml-auto")} {...props} />;
      case "macos":
        return <MacOS className={cn(customClass, justify && "ml-0")} {...props} />;
      case "gnome":
        return <Gnome className={cn(customClass, justify && "ml-auto", "pt-2.5")} {...props} />;
      default:
        return <Windows className={cn(customClass, justify && "ml-auto")} {...props} />;
    }
  };

  return (
    <TauriAppWindowProvider>
      <ControlsComponent />
    </TauriAppWindowProvider>
  );
}
