import { Icon } from "../../../../packages/moss-ui/src";
import { useEffect, useState } from "react";

// FIXME:
// The statically created array `tips` is a temporary solution.
// In the future, this data will be fetched from the backend.
const tips = [
  "The statusbar color can be changed in the appearance settings",
  "You can change the order of widget actions. Try it!",
  "Lorem ipsum dolor sit amat. Met the statusbar color can be changed in the appearance settings",
];

export const PageLoader = () => {
  const [tip, setTip] = useState(tips[0]);
  useEffect(() => {
    const interval = setInterval(() => {
      const randomTip = tips[Math.floor(Math.random() * tips.length)];
      setTip(randomTip);
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="relative flex h-full w-full flex-col items-center justify-between bg-white pt-4">
      <div className="fixed top-0 h-36 w-full" data-tauri-drag-region />

      <div className="flex h-full flex-col items-center justify-center gap-5">
        <Icon icon="Loader" className="size-8 animate-[spin_1s_ease-in-out_infinite]" />
        <div className="flex flex-col gap-3 text-center">
          <div className="font-black">Did you know</div>
          <div className="text animate-text-slide text-[#6F6F6F] ">{tip}</div>
        </div>
      </div>

      <div className="pb-4 text-xs text-[#525252]">This may take a few seconds.</div>
    </div>
  );
};
