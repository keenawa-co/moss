import { useEffect, useState } from "react";

const tips = [
  "The statusbar color can be changed in the appearance settings",
  "You can change the order of widget actions. Try it!",
  "Some other tip",
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
    <div className="relative flex h-screen w-screen flex-col items-center justify-between bg-white pt-4">
      <div className="fixed top-0 h-36 w-screen" data-tauri-drag-region />

      <div className="flex h-full flex-col items-center justify-center gap-5">
        <div className="animate-spin">
          <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="none" viewBox="0 0 32 32">
            <path
              fill="#C6C6C6"
              fill-rule="evenodd"
              d="M2 16c0-1.1046.89543-2 2-2h4c1.10457 0 2 .8954 2 2s-.89543 2-2 2H4c-1.10457 0-2-.8954-2-2Z"
              clip-rule="evenodd"
              opacity=".78"
            />
            <path
              fill="#C6C6C6"
              fill-rule="evenodd"
              d="M14 24c0-1.1046.8954-2 2-2s2 .8954 2 2v4c0 1.1046-.8954 2-2 2s-2-.8954-2-2v-4Z"
              clip-rule="evenodd"
              opacity=".62"
            />
            <path
              fill="#C6C6C6"
              fill-rule="evenodd"
              d="M22 16c0-1.1046.8954-2 2-2h4c1.1046 0 2 .8954 2 2s-.8954 2-2 2h-4c-1.1046 0-2-.8954-2-2Z"
              clip-rule="evenodd"
              opacity=".38"
            />
            <path
              fill="#C6C6C6"
              fill-rule="evenodd"
              d="M14 4c0-1.10457.8954-2 2-2s2 .89543 2 2v4c0 1.10457-.8954 2-2 2s-2-.89543-2-2V4Z"
              clip-rule="evenodd"
            />
            <path
              fill="#C6C6C6"
              fill-rule="evenodd"
              d="M6.10141 6.09976c.78105-.78105 2.04738-.78105 2.82843 0l2.82846 2.82843c.781.78105.781 2.04741 0 2.82841-.7811.7811-2.04741.7811-2.82846 0L6.10141 8.92819c-.78105-.78105-.78105-2.04738 0-2.82843Z"
              clip-rule="evenodd"
              opacity=".93"
            />
            <rect
              width="4"
              height="8"
              x="10.3438"
              y="18.8281"
              fill="#C6C6C6"
              opacity=".69"
              rx="2"
              transform="rotate(45 10.3438 18.8281)"
            />
            <path
              fill="#C6C6C6"
              fill-rule="evenodd"
              d="M20.242 20.2423c.7811-.781 2.0474-.781 2.8285 0l2.8284 2.8285c.781.781.781 2.0473 0 2.8284-.7811.781-2.0474.781-2.8284 0l-2.8285-2.8284c-.781-.7811-.781-2.0474 0-2.8285Z"
              clip-rule="evenodd"
              opacity=".48"
            />
            <path
              fill="#C6C6C6"
              fill-rule="evenodd"
              d="M23.0702 6.09976c.781-.78105 2.0473-.78105 2.8284 0 .781.78105.781 2.04738 0 2.82843l-2.8284 2.82841c-.7811.7811-2.0474.7811-2.8285 0-.781-.781-.781-2.04736 0-2.82841l2.8285-2.82843Z"
              clip-rule="evenodd"
              opacity=".3"
            />
          </svg>
        </div>
        <div className="flex flex-col gap-4 text-center">
          <div className="font-black">Did you know</div>
          <div className="text animate-text-slide text-[#6F6F6F] ">{tip}</div>
        </div>
      </div>

      <div className="pb-4 text-xs text-[#525252]">This may take a few seconds.</div>
    </div>
  );
};
