import { useState } from "react";

import * as icons from "@repo/icongen";
import type { Meta, StoryObj } from "@storybook/react";

import { Icon, Icons } from "./Icon";
import { cn } from "./utils";

const iconOptions = Object.keys(icons) as Icons[];

const meta: Meta<typeof Icon> = {
  title: "Shared/Icon",
  component: Icon,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
  args: {
    className: "text-6xl",
  },
  argTypes: {
    icon: { control: { type: "select" }, options: iconOptions },
  },
} satisfies Meta<typeof Icon>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Gallery: Story = {
  parameters: {
    layout: "fullscreen",
  },
  args: {
    icon: null, // Default value for the `icon` argument
    className: "text-6xl", // Match defaults if necessary
  },
  render: () => <GalleryComponent />,
};

const GalleryComponent = () => {
  const [search, setSearch] = useState("");

  const filteredIcons = () => {
    if (search === "") {
      return Object.entries(icons);
    }
    return Object.entries(icons).filter(([name]) => {
      return name.toLowerCase().includes(search.toLowerCase());
    });
  };

  const [lastChosenSizeType, setLastChosenSizeType] = useState<"TW" | "px" | undefined>(undefined);
  const handleResetButton = () => {
    setAllIconsSize("16");
    setAllIconsSizeTW("text-base");
    setLastChosenSizeType(undefined);
  };

  //all sizes
  const [allIconsSize, setAllIconsSize] = useState("16");
  const handleAllIconsSizeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setAllIconsSize(e.target.value);
    setLastChosenSizeType("px");
  };

  const [allIconsSizeTW, setAllIconsSizeTW] = useState("text-base");
  const handleAllIconsSizeTWChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setAllIconsSizeTW(e.target.value);
    setLastChosenSizeType("TW");
  };
  const tailwindSizes = [
    "text-xs",
    "text-sm",
    "text-base",
    "text-lg",
    "text-xl",
    "text-2xl",
    "text-3xl",
    "text-4xl",
    "text-5xl",
    "text-6xl",
    "text-7xl",
    "text-8xl",
    "text-9xl",
  ];

  //all colors
  const [allColors, setAllColors] = useState("#808080");
  const handleAllColorsChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setAllColors(e.target.value);
  };

  const computedClassName = () => {
    let classes = [];

    if (allColors !== "#808080") {
      classes.push(`text-[${allColors}]`);
    }

    if (lastChosenSizeType === "TW") {
      classes.push(allIconsSizeTW);
    }

    if (lastChosenSizeType === "px") {
      classes.push(`text-${allIconsSize}px`);
    }

    return classes.join(" ");
  };

  const [theme, setTheme] = useState<"light" | "dark">("light");

  return (
    <div className={cn("h-screen overflow-auto px-4 pt-4 pb-12", theme === "light" ? "" : "bg-[#161819] text-white")}>
      <div className="flex gap-6">
        <div className="flex w-full flex-col gap-6">
          <input
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            type="text"
            className={cn(
              "w-full rounded border border-[#E0E0E0] px-2 py-1 font-medium text-black placeholder-[#A8ADBD] hover:bg-[#EAEAEA] focus:bg-[#C6C6C6] focus:outline-hidden focus-visible:outline-hidden active:outline-hidden",
              theme === "light" ? "" : "bg-[#1E2021] text-black"
            )}
            placeholder={`Search for ${Object.keys(icons).length} icons`}
          />

          <div className="grid w-full grid-cols-1 justify-items-center gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 xl:grid-cols-5 2xl:grid-cols-6">
            {filteredIcons().map(([name, Icon]) => (
              <div className={cn("flex size-40 flex-col rounded-lg bg-white", theme === "light" ? "" : "bg-[#1E2021]")}>
                <div className={cn(`grid grow place-items-center`)}>
                  <Icon
                    className={lastChosenSizeType === "TW" ? allIconsSizeTW : ""}
                    style={
                      lastChosenSizeType === "px"
                        ? { fontSize: `${allIconsSize}px`, color: allColors }
                        : { color: allColors }
                    }
                  />
                </div>
                <div className="w-full text-center text-xs font-semibold break-all">{name}</div>
              </div>
            ))}
          </div>
        </div>

        <div className="flex min-w-60 flex-col gap-6 p-4">
          <div>
            <button onClick={() => setTheme(theme === "light" ? "dark" : "light")}>
              {theme === "light" ? (
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 30.457 30.457">
                  <path d="M29.693 14.49a1.18 1.18 0 0 0-1.32.353 9.37 9.37 0 0 1-7.27 3.446c-5.183 0-9.396-4.216-9.396-9.397a9.3 9.3 0 0 1 2.963-6.835 1.18 1.18 0 0 0 .264-1.343A1.171 1.171 0 0 0 13.75.03 15.172 15.172 0 0 0 0 15.194c0 8.402 6.836 15.238 15.238 15.238 8.303 0 14.989-6.506 15.219-14.812a1.169 1.169 0 0 0-.764-1.13z" />
                </svg>
              ) : (
                <svg
                  fill="white"
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 292.548 292.548"
                >
                  <path d="M221.253 146.83c0 39.842-32.396 72.231-72.223 72.231-39.839 0-72.238-32.401-72.238-72.231 0-39.833 32.405-72.231 72.238-72.231 39.821-.001 72.223 32.403 72.223 72.231zM149.03 47.105c3.984 0 7.221-3.239 7.221-7.224V9.776a7.217 7.217 0 0 0-7.221-7.224 7.224 7.224 0 0 0-7.23 7.224v30.105a7.224 7.224 0 0 0 7.23 7.224zm71.887 36.716a7.18 7.18 0 0 0 5.104-2.114l25.881-25.875c2.822-2.832 2.822-7.41 0-10.226a7.232 7.232 0 0 0-10.208 0l-25.881 25.887c-2.822 2.828-2.822 7.397 0 10.214a7.183 7.183 0 0 0 5.104 2.114zM60.504 81.708a7.24 7.24 0 0 0 5.104 2.114 7.22 7.22 0 0 0 5.116-12.328L44.832 45.607a7.232 7.232 0 0 0-10.208 0c-2.822 2.822-2.822 7.395 0 10.226l25.88 25.875zm82.759 163.739c-3.99 0-7.218 3.242-7.218 7.224v30.102c0 3.987 3.233 7.224 7.218 7.224s7.232-3.23 7.232-7.224V252.67c0-3.981-3.242-7.223-7.232-7.223zm-77.003-34.6-25.88 25.88c-2.822 2.822-2.822 7.398 0 10.208a7.223 7.223 0 0 0 5.116 2.12c1.852 0 3.69-.702 5.104-2.12l25.881-25.88c2.822-2.822 2.822-7.398 0-10.208a7.24 7.24 0 0 0-10.221 0zm165.525 0a7.23 7.23 0 0 0-10.214 0 7.212 7.212 0 0 0 0 10.208l25.881 25.88a7.204 7.204 0 0 0 5.115 2.12c1.85 0 3.688-.702 5.099-2.12a7.212 7.212 0 0 0 0-10.208l-25.881-25.88zM46.96 146.83c0-3.996-3.249-7.224-7.233-7.224H7.218A7.215 7.215 0 0 0 0 146.83a7.22 7.22 0 0 0 7.218 7.224h32.51c3.99-.001 7.232-3.231 7.232-7.224zm238.364-7.224h-38.527a7.217 7.217 0 0 0-7.218 7.224 7.218 7.218 0 0 0 7.218 7.224h38.527a7.223 7.223 0 0 0 7.224-7.224 7.22 7.22 0 0 0-7.224-7.224z" />
                </svg>
              )}
            </button>
          </div>
          <div className="flex flex-col gap-1.5">
            <div className="min-h-5 w-full bg-[#1c2021]">
              <span className="text-[#ffb224]">className</span>
              <span className="text-[#6bc084]">=</span>
              <span className="text-[#aa997f]">"</span>
              <span className="text-[#9bbb2a]">{computedClassName()}</span>
              <span className="text-[#aa997f]">"</span>
            </div>
            <div className="min-h-5 w-full bg-[#1c2021]">
              <span className="text-[#9bbb2a]">{computedClassName()}</span>
            </div>
          </div>
          <div className="flex justify-between">
            <span className="font-bold">Customize</span>
            <button
              onClick={handleResetButton}
              className="rounded-[4px] bg-[#0065FF] px-3 py-1 font-medium text-white hover:bg-[#0052CC] active:bg-[#002D9C]"
            >
              Reset
            </button>
          </div>

          <div className="flex flex-col gap-1.5">
            <div className="flex w-full items-center justify-between">
              <h3 className={cn(theme === "light" ? "text-black" : "text-white")}>Size</h3>
              <div>{lastChosenSizeType === "TW" ? `${allIconsSizeTW}` : `${allIconsSize}px`}</div>
            </div>

            <div className="flex w-full items-center gap-3">
              <input
                type="range"
                min={4}
                max={100}
                value={allIconsSize}
                onChange={handleAllIconsSizeChange}
                className="w-full"
              />
              <div className="font-semibold">{allIconsSize}px</div>
            </div>

            <div className="w-full">
              <select
                value={allIconsSizeTW}
                onChange={handleAllIconsSizeTWChange}
                className={cn("focus:ring-2 focus:ring-[#3574F0]", theme === "light" ? "" : "bg-[#1E2021] text-white")}
              >
                {tailwindSizes.map((size, index) => (
                  <option key={index} value={size} label={size} />
                ))}
              </select>
            </div>
          </div>

          <div className="flex flex-col">
            <div className="flex w-full items-center justify-between">
              <h3 className={cn(theme === "light" ? "text-black" : "text-white")}>Color</h3>
            </div>
            <div className="flex items-center justify-between">
              <div>{allColors}</div>
              <input type="color" value={allColors} onChange={handleAllColorsChange} />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export const AllVariants: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl",
  },
  render: (args) => {
    return (
      <div className="flex w-full flex-col gap-6 p-16">
        <h2 className="text-3xl font-semibold">All variants</h2>

        <div className="grid grid-cols-4 justify-items-center gap-4">
          <div>Default</div>
          <div>Fill</div>
          <div>Stroke</div>
          <div>Without default color</div>
          <Icon icon={args.icon} className="text-6xl" />
          <Icon icon={args.icon} className="text-6xl text-green-400" />
          <Icon icon={args.icon} className="stroke-green-400 stroke-1 text-6xl" viewBox="-1 -1 18 17" />
          <Icon icon="NewProject" className="text-6xl" />
        </div>
      </div>
    );
  },
};

export const Default: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl",
  },
};

export const Stroke: Story = {
  args: {
    icon: "Goals",
    className: "text-6xl stroke-1 stroke-red-500",
    viewBox: "0 0 20 20",
  },
};

export const Fill: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl text-green-300",
  },
};

export const WithoutDefaultColor: Story = {
  args: {
    icon: "NewProject",
    className: "text-6xl text-red-300",
  },
};
