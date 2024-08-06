import type { Meta, StoryObj } from "@storybook/react";
import Tooltip from "./Tooltip";
import "./styles.css";

const meta = {
  title: "Shared/Tooltip",
  component: Tooltip,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tooltip>;

export default meta;
type Story = StoryObj<typeof meta>;

const TooltipTemplate = (args: any) => (
  <Tooltip {...args}>
    <button>Tooltip</button>
  </Tooltip>
);

export const WithShortcut: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: "Settings",
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const NoShortcut: Story = {
  args: {
    label: "Settings",
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const ALotOfText: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: 'Not a lorem ipsum text, because spell checker marks words from lorem ipsum text as "problems"',
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const WithArrow: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: "Settings",
    options: {
      arrow: {},
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const SideBottom: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: "Settings",
    options: {
      content: {
        side: "bottom",
      },
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const AlignStart: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: "Settings",
    options: {
      content: {
        align: "start",
      },
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const InstantOpen: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: "Settings",
    options: {
      provider: {
        delayDuration: 0,
      },
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const AlwaysOpen: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: "Settings",
    options: {
      root: {
        open: true,
      },
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const CustomClassName: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: 'Not a lorem ipsum text, because spell checker marks words from lorem ipsum text as "problems"',
    className: "bg-red-500 text-blue-600 p-6 max-w-full text-2xl font-bold italic",
  },
  render: (args) => <TooltipTemplate {...args} />,
};
