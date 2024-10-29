import { useEffect, useState } from "react";
import { cn } from "@/components/window-controls/libs/utils";
import { TauriAppWindowProvider } from "@/components/window-controls/contexts";
import { Gnome, MacOS, Windows } from "@/components/window-controls/controls";
import { getOsType } from "@/components/window-controls/libs/plugin-os";
import type { WindowControlsProps } from "@/components/window-controls/types";
import { OsType } from "@tauri-apps/plugin-os";

export function WindowControls({
  platform,
  justify = false,
  hide = false,
  hideMethod = "display",
  className,
  ...props
}: WindowControlsProps) {
  const [osType, setOsType] = useState<OsType | undefined>(undefined);

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
        platform = "linux";
        break;
      default:
        platform = "windows";
    }
  }

  const ControlsComponent = () => {
    switch (platform) {
      case "windows":
        return <Windows className={cn(customClass)} {...props} />;
      case "macos":
        return <MacOS className={cn(customClass)} {...props} />;
      case "linux":
        return <Gnome className={cn(customClass, "py-2.5")} {...props} />;
      default:
        return <Windows className={cn(customClass)} {...props} />;
    }
  };

  return (
    <TauriAppWindowProvider>
      <ControlsComponent />
    </TauriAppWindowProvider>
  );
}
