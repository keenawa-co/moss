import type { Meta, StoryObj } from "@storybook/react";
import { ResizablePanelGroup, ResizablePanel, ResizableHandle } from "./Resizable";
import { render } from "react-dom";

// api reference https://github.com/bvaughn/react-resizable-panels/tree/main/packages/react-resizable-panels#props
// more examples https://react-resizable-panels.vercel.app/

const meta = {
  title: "desktop/Resizable",
  component: ResizablePanelGroup,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
  args: {},
  decorators: [
    (Story) => {
      return (
        <div className="min-h-20 text-white">
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
      <ResizablePanelGroup direction="horizontal" className="bg-slate-500 text-center h-full ">
        <ResizablePanel className="h-44 w-1/2 grid items-center">
          <div>One</div>
        </ResizablePanel>
        <ResizableHandle className="w-px bg-sky-400" />
        <ResizablePanel className="h-44 w-1/2 grid items-center">
          <div>Two</div>
        </ResizablePanel>
      </ResizablePanelGroup>
    );
  },
};

export const Vertical: Story = {
  parameters: {
    layout: "padded",
  },
  render: () => {
    return (
      <ResizablePanelGroup direction="vertical" className="bg-slate-500 text-center  min-h-44 ">
        <ResizablePanel>
          <div className=" w-full h-full grid items-center">One</div>
        </ResizablePanel>

        <ResizableHandle className="h-px bg-sky-400" />

        <ResizablePanel>
          <div className="w-full h-full grid items-center ">Two</div>
        </ResizablePanel>
      </ResizablePanelGroup>
    );
  },
};

export const CollapsingPanel: Story = {
  render: () => {
    return (
      <ResizablePanelGroup direction="horizontal" className="bg-slate-500 text-center h-full">
        <ResizablePanel className="h-44 w-1/2 grid items-center" collapsible minSize={15}>
          <div>Collapsible</div>
        </ResizablePanel>
        <ResizableHandle className="w-px bg-sky-400" />
        <ResizablePanel className="h-44 w-1/2 grid items-center">
          <div>Not Collapsible</div>
        </ResizablePanel>
      </ResizablePanelGroup>
    );
  },
};

export const MinMax: Story = {
  render: () => {
    return (
      <ResizablePanelGroup direction="horizontal" className="bg-slate-500 text-center h-full">
        <ResizablePanel className="h-44 w-1/2 grid items-center" minSize={10} maxSize={60}>
          <div>
            Min width: <b>10%</b> <b className="text-black">|</b> Max width: <b>60%</b>
          </div>
        </ResizablePanel>
        <ResizableHandle className="w-px bg-sky-400" />
        <ResizablePanel className="h-44 w-1/2 grid items-center">
          <div>Two</div>
        </ResizablePanel>
      </ResizablePanelGroup>
    );
  },
};

export const Persistent: Story = {
  render: () => {
    return (
      <ResizablePanelGroup direction="horizontal" className="bg-slate-500 text-center h-full " autoSaveId="storybook">
        <ResizablePanel className="h-44 w-1/2 grid items-center">
          <div>This handle position will be saved in local storage</div>
        </ResizablePanel>
        <ResizableHandle className="w-px bg-sky-400" />
        <ResizablePanel className="h-44 w-1/2 grid items-center">
          <div>Two</div>
        </ResizablePanel>
      </ResizablePanelGroup>
    );
  },
};
