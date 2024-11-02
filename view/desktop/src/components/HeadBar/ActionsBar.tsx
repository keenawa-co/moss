import { RootState, useAppDispatch } from "@/store";
import { toggleSidebarVisibility } from "@/store/sidebar/sidebarSlice";
import { Icon } from "@repo/ui";
import { HTMLProps } from "react";
import { useSelector } from "react-redux";
import { HeadBarButton } from "./HeadBarButton";

export const ActionsBar = (props: HTMLProps<HTMLDivElement>) => {
  const dispatch = useAppDispatch();
  const isSidebarVisible = useSelector((state: RootState) => state.sidebar.sidebarVisible);

  return (
    <div className="flex items-center gap-2" {...props}>
      <div className="flex items-center">
        <button className="flex items-center gap-[1px] transition-colors">
          <div className="flex h-full items-center gap-[6px] rounded py-[9px] pl-[10px] pr-[8px] hover:bg-[#D3D3D3] ">
            <Icon icon="HeadBarBranch" className="size-[18px] text-[#525252]" />
            <div className="flex items-center gap-[2px]">
              <span className=" leading-4 text-[#161616]">main</span>
              <span className="rounded bg-[#C6C6C6] px-1 text-xs font-semibold text-[#525252]">#50</span>
            </div>
          </div>

          <div className="flex items-center gap-1 pr-[10px]">
            <Icon icon="HeadBarBranchSuccess" className="size-[16px] cursor-default rounded" />
            <Icon icon="HeadBarBranchRefresh" className="size-[16px] cursor-default rounded text-[#525252]" />
            <Icon icon="ArrowheadDown" className="size-[16px] cursor-default rounded text-[#525252]" />
          </div>
        </button>
      </div>

      <div className="flex items-center gap-0.5">
        <HeadBarButton
          icon={isSidebarVisible ? "HeadBarPrimarySideBarActive" : "HeadBarPrimarySideBar"}
          className="p-[6px] "
          iconClassName="w-[16px] h-[14px]"
          onClick={() => dispatch(toggleSidebarVisibility({}))}
        />
        <HeadBarButton icon="HeadBarPanelActive" className="p-[6px]  " iconClassName="w-[16px] h-[14px]" />
        <HeadBarButton icon="HeadBarSecondarySideBar" className="p-[6px]  " iconClassName="w-[16px] h-[14px]" />
        <HeadBarButton icon="HeadBarCustomizeLayout" className="p-[6px]  " iconClassName="w-[16px] h-[14px]" />
      </div>

      <Separator />

      <div className="flex items-center gap-1">
        <HeadBarButton icon="HeadBarAccount" className="p-[4px]" iconClassName="size-[18px]" />
        <HeadBarButton icon="HeadBarNotifications" className="p-[4px]" iconClassName="size-[18px]" />
        <HeadBarButton icon="HeadBarWrench" className="p-[4px]" iconClassName="size-[18px]" />
      </div>
    </div>
  );
};

const Separator = () => <div className="separator h-[15px] w-px bg-[#C6C6C6]" />;
