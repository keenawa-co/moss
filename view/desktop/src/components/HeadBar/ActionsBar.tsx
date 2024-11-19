import { RootState, useAppDispatch } from "@/store";
import { toggleSidebarVisibility } from "@/store/sidebar/sidebarSlice";
import { cn, Icon } from "@repo/ui";
import { HTMLProps } from "react";
import { useSelector } from "react-redux";
import { ActionsGroup } from "../ActionsGroup";

export const ActionsBar = ({ className, ...props }: HTMLProps<HTMLDivElement>) => {
  const dispatch = useAppDispatch();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);

  return (
    <div className={cn("flex items-center gap-3", className)} {...props}>
      <div className="flex items-center">
        <button className="flex items-center gap-px transition-colors">
          <div className="flex h-full items-center gap-1.5 rounded py-1.5 pl-2.5 pr-2 hover:bg-[#D3D3D3]">
            <Icon icon="HeadBarBranch" className="size-[18px] text-[#525252]" />
            <div className="flex items-center gap-0.5">
              <span className="leading-4 text-[#161616]">main</span>
              <span className="rounded bg-[#C6C6C6] px-1 text-xs font-semibold text-[#525252]">#50</span>
            </div>
          </div>

          <div className="flex cursor-default items-center gap-1 pr-2.5">
            <Icon icon="HeadBarBranchSuccess" className="size-4 rounded" />
            <Icon icon="HeadBarBranchRefresh" className="size-4 rounded text-[#525252]" />
            <Icon icon="ArrowheadDown" className="size-4 rounded text-[#525252]" />
          </div>
        </button>
      </div>

      <div className="flex items-center">
        <ActionsGroup
          icon={isSidebarVisible ? "HeadBarPrimarySideBarActive" : "HeadBarPrimarySideBar"}
          onClick={() => dispatch(toggleSidebarVisibility({}))}
          iconClassName="size-[18px]"
          className="size-[30px] "
        />
        <ActionsGroup icon="HeadBarPanelActive" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarSecondarySideBar" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarCustomizeLayout" className="size-[30px] " iconClassName="size-[18px]" />
      </div>

      <div className="flex items-center">
        <ActionsGroup icon="HeadBarAccount" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarNotifications" className="size-[30px] " iconClassName="size-[18px]" />
        <ActionsGroup icon="HeadBarWrench" className="size-[30px] " iconClassName="size-[18px]" />
      </div>
    </div>
  );
};
