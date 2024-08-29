import type { Meta, StoryObj } from "@storybook/react";
import { render } from "react-dom";
import { Resizable, ResizablePanel } from "./Resizable";
import "allotment/dist/style.css";
import { useState } from "react";
import { cn } from "@/shared/utils/utils";

const meta = {
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
  render: () => {
    return (
      <Resizable vertical>
        <ResizablePanel className="grid place-items-center">One</ResizablePanel>
        <ResizablePanel className="grid place-items-center">Two</ResizablePanel>
      </Resizable>
    );
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
  render: () => {
    const [oneVisible, setOneVisible] = useState(true);
    const [twoVisible, setTwoVisible] = useState(true);
    const [threeVisible, setThreeVisible] = useState(true);
    const [fourVisible, setFourVisible] = useState(true);

    return (
      <>
        <div className="flex gap-2 p-4 ">
          <button
            className={cn(`border rounded p-2 duration-0`, { "bg-sky-500 border-sky-500 text-white": oneVisible })}
            onClick={() => setOneVisible(!oneVisible)}
          >
            One
          </button>
          <button
            className={cn(`border rounded p-2 duration-0`, { "bg-sky-500 border-sky-500 text-white": twoVisible })}
            onClick={() => setTwoVisible(!twoVisible)}
          >
            Two
          </button>
          <button
            className={cn(`border rounded p-2 duration-0`, { "bg-sky-500 border-sky-500 text-white": threeVisible })}
            onClick={() => setThreeVisible(!threeVisible)}
          >
            Three
          </button>
          <button
            className={cn(`border rounded p-2 duration-0`, { "bg-sky-500 border-sky-500 text-white": fourVisible })}
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
  },
};
