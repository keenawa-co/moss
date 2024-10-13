import { Icon } from "@repo/ui";
import { useState } from "react";

export const SidebarGeneral = () => {
  const [input, setInput] = useState("");
  return (
    <div>
      <div className="flex cursor-pointer items-center gap-2 text-[14px] font-bold">
        <div>
          <Icon icon="Clock" className="size-[18px]" />
        </div>
        <span>Recents</span>
      </div>
    </div>
  );
};
