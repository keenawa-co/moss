import type { Meta, StoryObj } from "@storybook/react";
import { WindowTitlebar } from "@/components/WindowTitlebar";
import { Menu, LogoSvg } from "@/components/test-components/TestComponents";
import "@/assets/index.css";

const meta = {
  title: "desktop/WindowTitlebar",
  component: WindowTitlebar,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof WindowTitlebar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
  render: (args) => <WindowTitlebar {...args} />,
};

export const WithContent: Story = {
  args: {
    className: "border",
  },
  render: (args) => <WindowTitlebar {...args}>Content</WindowTitlebar>,
};

export const WithMacControlsFullWidth: Story = {
  args: {
    controlsOrder: "right",
    className: "grid grid-cols-2 align-center h-10 rounded-t-lg border border-dashed border-slate-400 bg-white shadow",
    windowControlsProps: {
      platform: "macos",
    },
  },
  render: (args) => (
    <WindowTitlebar className="w-full border" {...args}>
      content
    </WindowTitlebar>
  ),
};

export const MacControlsOnRightSide: Story = {
  args: {
    controlsOrder: "right",
    windowControlsProps: {
      platform: "macos",
      className: "bg-rose-300 rounded-full p-2",
    },
  },
  render: (args) => (
    <WindowTitlebar {...args}>
      <div className="bg-sky-200 flex items-center justify-center rounded-full px-2">
        titlebar content without w-full (macos but on the right side)
      </div>
    </WindowTitlebar>
  ),
};

export const StyledWindowsControls: Story = {
  args: {
    controlsOrder: "right",
    className: "grid grid-cols-2 align-center h-10 rounded-t-lg border border-dashed border-slate-400 bg-white shadow",
    windowControlsProps: {
      platform: "windows",
      className: "relative bg-yellow-300 rounded-full",
    },
  },
  render: (args) => (
    <WindowTitlebar {...args}>
      <div className="flex items-center justify-center rounded-full bg-sky-500 px-2">
        titlebar content without w-full (windows but on the left side)
      </div>
    </WindowTitlebar>
  ),
};

export const WithIconAndTitle: Story = {
  args: {
    controlsOrder: "platform",
    className:
      "h-10 rounded-t-lg border border-dashed border-slate-400  bg-white shadow dark:border-slate-600 dark:bg-slate-800",
    windowControlsProps: {
      platform: "windows",
      className: "",
    },
  },
  render: (args) => (
    <WindowTitlebar {...args}>
      <div className="mr-96 flex h-full w-full items-center justify-center" data-tauri-drag-region>
        <LogoSvg />
        App Title
      </div>
    </WindowTitlebar>
  ),
};

export const WithIconMenuAndTitle: Story = {
  args: {
    controlsOrder: "platform",
    className: "h-10 rounded-t-lg  bg-white shadow dark:bg-slate-800",
    windowControlsProps: {
      platform: "windows",
    },
  },
  render: (args) => (
    <WindowTitlebar {...args}>
      <div className="ml-3 flex items-center" data-tauri-drag-region>
        <LogoSvg />
        <Menu />
      </div>

      <div className="mx-20 flex items-center justify-center" data-tauri-drag-region>
        App Title
      </div>
    </WindowTitlebar>
  ),
};
