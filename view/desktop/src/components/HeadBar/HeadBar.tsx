import { WindowControls } from "../window-controls/WindowControls";
import { HeadBarButton } from "./HeadBarButton";
import { type } from "@tauri-apps/plugin-os";
import { HeadBarDropdown } from "./HeadBarDropdown";
import { cn } from "@repo/ui";

export const HeadBar = () => {
  let os = type();

  // os = "macos";
  // os = "linux";

  return (
    <header
      data-tauri-drag-region
      className={cn("flex h-full max-h-[46px] items-center border-b border-solid border-[#C6C6C6] bg-[#E0E0E0]")}
    >
      {os === "macos" && <WindowControls platform={os} />}

      <div
        className={cn("flex grow items-center", {
          "pl-[10px] pr-[16px]": os === "macos",
          "px-[16px]": os === "windows" || os === "linux",
        })}
      >
        <HeadBarDropdown icon="HeadBarMossStudio" label="moss-studio" />

        <Separator />

        <div className="flex w-full justify-between" data-tauri-drag-region>
          <div className="flex items-center gap-4">
            <HeadBarButton icon="HeadBarAlerts" label="Alerts" />
            <HeadBarButton icon="HeadBarDiscovery" label="Discovery" />
            <HeadBarButton icon="HeadBarCommunity" label="Community" />
          </div>

          <div className="flex items-center gap-4">
            {/* <HeadBarButton icon="HeadBarBranch" label="moss" /> */}
            <HeadBarDropdown icon="HeadBarBranch" label="moss" />
            <HeadBarButton icon="HeadBarTogglePrimarySideBar" />
            <HeadBarButton icon="HeadBarTogglePanel" />
            <HeadBarButton icon="HeadBarToggleSecondarySideBar" />
            <HeadBarButton icon="HeadBarCustomizeLayout" />
          </div>
        </div>

        <Separator />

        <div className="flex items-center gap-4">
          <HeadBarButton icon="HeadBarAccount" />
          <HeadBarButton icon="HeadBarNotifications" />
          <HeadBarButton icon="HeadBarSettings" />
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <WindowControls platform={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <WindowControls />}
    </header>
  );
};

const Separator = () => <div className="separator mx-3 h-[15px] w-px bg-[#C6C6C6]" />;
