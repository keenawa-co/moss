import { Meta, StoryObj } from "@storybook/react";

import { Resizable, ResizablePanel } from "./Resizable";

import "allotment/dist/style.css";

import { useState } from "react";

import { cn } from "@repo/moss-ui";
import { expect, within } from "@storybook/test";

const meta: Meta = {
  title: "desktop/Resizable",
  component: Resizable,
  tags: ["autodocs"],
  parameters: {
    layout: "fullscreen",
  },
  args: {},
  decorators: [
    (Story) => {
      return (
        <div className="h-96 border border-dashed">
          <Story />
        </div>
      );
    },
  ],
} satisfies Meta<typeof Resizable>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    return (
      <Resizable>
        <ResizablePanel className="grid place-items-center">One</ResizablePanel>
        <ResizablePanel className="grid place-items-center">Two</ResizablePanel>
      </Resizable>
    );
  },
};

export const Vertical: Story = {
  tags: ["stable"],
  render: () => {
    return (
      <Resizable vertical>
        <ResizablePanel className="grid place-items-center">One</ResizablePanel>
        <ResizablePanel className="grid place-items-center">Two</ResizablePanel>
      </Resizable>
    );
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    expect(canvas.getByText("One")).toBeInTheDocument();
  },
};

export const CollapsingPanel: Story = {
  render: () => {
    return (
      <Resizable>
        <ResizablePanel className="grid place-items-center" minSize={200} snap>
          Collapsible
        </ResizablePanel>
        <ResizablePanel className="grid place-items-center">Two</ResizablePanel>
      </Resizable>
    );
  },
};

export const MinMax: Story = {
  render: () => {
    return (
      <Resizable>
        <ResizablePanel minSize={100} maxSize={600} className="grid place-items-center">
          <span className="text-center">
            MinMax: <b>100 - 600px</b>
          </span>
        </ResizablePanel>
        <ResizablePanel className="grid place-items-center">Two</ResizablePanel>
      </Resizable>
    );
  },
};

export const Nested: Story = {
  render: () => {
    return (
      <Resizable>
        <ResizablePanel className="grid place-items-center" preferredSize={200} maxSize={400} minSize={100} snap>
          Sidebar
        </ResizablePanel>
        <ResizablePanel className="grid place-items-center">
          <Resizable vertical>
            <ResizablePanel className="grid place-items-center">code</ResizablePanel>
            <ResizablePanel className="grid place-items-center" minSize={100} snap>
              terminal
            </ResizablePanel>
          </Resizable>
        </ResizablePanel>
      </Resizable>
    );
  },
};

export const Controls: Story = {
  render: () => <ControlsStory />,
};

export const ControlsStory = () => {
  const [oneVisible, setOneVisible] = useState(true);
  const [twoVisible, setTwoVisible] = useState(true);
  const [threeVisible, setThreeVisible] = useState(true);
  const [fourVisible, setFourVisible] = useState(true);

  return (
    <>
      <div className="flex gap-2 p-4">
        <button
          className={cn(`rounded border p-2 duration-0`, { "border-sky-500 bg-sky-500 text-white": oneVisible })}
          onClick={() => setOneVisible(!oneVisible)}
        >
          One
        </button>
        <button
          className={cn(`rounded border p-2 duration-0`, { "border-sky-500 bg-sky-500 text-white": twoVisible })}
          onClick={() => setTwoVisible(!twoVisible)}
        >
          Two
        </button>
        <button
          className={cn(`rounded border p-2 duration-0`, { "border-sky-500 bg-sky-500 text-white": threeVisible })}
          onClick={() => setThreeVisible(!threeVisible)}
        >
          Three
        </button>
        <button
          className={cn(`rounded border p-2 duration-0`, { "border-sky-500 bg-sky-500 text-white": fourVisible })}
          onClick={() => setFourVisible(!fourVisible)}
        >
          Four
        </button>
      </div>
      <div className="h-80 pb-2">
        <Resizable vertical>
          <ResizablePanel className="grid place-items-center">
            <Resizable>
              <ResizablePanel className="grid place-items-center" visible={oneVisible}>
                One
              </ResizablePanel>
              <ResizablePanel className="grid place-items-center" visible={twoVisible}>
                Two
              </ResizablePanel>
              <ResizablePanel className="grid place-items-center" visible={threeVisible}>
                Three
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>
          <ResizablePanel className="grid place-items-center" visible={fourVisible}>
            Four
          </ResizablePanel>
        </Resizable>
      </div>
    </>
  );
};

export const DotsOnTextOverflow: Story = {
  render: () => {
    return (
      <Resizable>
        <ResizablePanel className="grid place-items-center" preferredSize={100}>
          <div className="w-full overflow-hidden">
            {Array(10)
              .fill(0)
              .map((_, i) => (
                <div key={i} className="overflow-hidden text-ellipsis whitespace-nowrap">
                  Menu item - {i + 1} with a very long name
                </div>
              ))}
          </div>
        </ResizablePanel>
        <ResizablePanel className="grid place-items-center">
          <ol className="list-decimal text-xl">
            <li>
              <span className="font-black">Container</span> with your items
              <span className="font-black"> must have </span>
              <u>overflow: hidden</u> <span>and a </span>
              <u>fixed width or width:100%</u>
            </li>
            <li>
              Your <span className="font-black">items must have </span>
              <u>text-ellipsis</u> <span>and </span>
              <u>whitespace-nowrap</u> <span>and </span>
              <u>overflow:hidden</u>
            </li>
          </ol>
        </ResizablePanel>
      </Resizable>
    );
  },
};

export const OverflowXY: Story = {
  render: () => {
    return (
      <Resizable>
        <ResizablePanel className="grid place-items-center" preferredSize={200} maxSize={300} snap>
          <div className="h-full w-full overflow-y-scroll">
            <div className="mb-4 mt-12 h-12 text-center font-bold">Overflow Y</div>
            {Array(10)
              .fill(0)
              .map((_, i) => (
                <div key={i} className="h-12 overflow-hidden text-ellipsis whitespace-nowrap">
                  Menu item - {i + 1} with a very long name
                </div>
              ))}
          </div>
        </ResizablePanel>

        <ResizablePanel maxSize={500}>
          <div className="flex h-full w-full flex-col justify-center overflow-x-scroll">
            <div>Overflow X</div>
            <div className="flex">
              <div className="h-44 w-full min-w-80 bg-[#e50000]" />
              <div className="h-44 w-full min-w-80 bg-[#ff8d00]" />
              <div className="h-44 w-full min-w-80 bg-[#ffee00]" />
              <div className="h-44 w-full min-w-80 bg-[#028121]" />
              <div className="h-44 w-full min-w-80 bg-[#004cff]" />
              <div className="h-44 w-full min-w-80 bg-[#770088]" />
            </div>
          </div>
        </ResizablePanel>

        <ResizablePanel>
          <div className="grid h-full w-full grid-cols-[repeat(6,minmax(320px,1fr))] overflow-auto">
            <div className="h-full min-h-80 w-full min-w-80 bg-[#e50000]">Overflow auto</div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#ff8d00]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#ffee00]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#028121]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#004cff]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#770088]"></div>

            <div className="h-full min-h-80 w-full min-w-80 bg-[#770088]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#004cff]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#028121]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#ffee00]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#ff8d00]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#e50000]"></div>

            <div className="h-full min-h-80 w-full min-w-80 bg-[#e50000]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#ff8d00]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#ffee00]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#028121]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#004cff]"></div>
            <div className="h-full min-h-80 w-full min-w-80 bg-[#770088]"></div>
          </div>
        </ResizablePanel>
      </Resizable>
    );
  },
};
