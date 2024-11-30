import { OsType, type } from "@tauri-apps/plugin-os";
import { MacOSControls } from "./MacOSControls";
import { LinuxControls } from "./LinuxControls";
import { WindowsControls } from "./WindowsControls";
import { cn } from "@repo/moss-ui";
import { HTMLProps } from "react";
import { TauriAppWindowProvider } from "./ControlsContext";

interface ControlsProps extends HTMLProps<HTMLDivElement> {
  os?: OsType;
  className?: string;
}

export const Controls = ({ os, className, ...props }: ControlsProps) => {
  const osFromTauri = type();

  const switchValue = os || osFromTauri;

  const ControlsComponent = () => {
    switch (switchValue) {
      case "windows":
        return <WindowsControls className={cn(className)} {...props} />;
      case "macos":
        return <MacOSControls className={cn(className)} {...props} />;
      case "linux":
        return <LinuxControls className={cn(className, "py-2.5")} {...props} />;
      default:
        return <WindowsControls className={cn(className)} {...props} />;
    }
  };

  return (
    <TauriAppWindowProvider>
      <ControlsComponent />
    </TauriAppWindowProvider>
  );
};
