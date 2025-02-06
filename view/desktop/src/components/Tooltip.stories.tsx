import type { Args, Meta, StoryObj } from "@storybook/react";

import Tooltip from "./Tooltip";

const meta = {
  title: "Desktop/Tooltip",
  component: Tooltip,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tooltip>;

export default meta;
type Story = StoryObj<typeof meta>;

const TooltipTemplate = (args: Args) => <Tooltip {...args}>Hover me!</Tooltip>;

export const Full: Story = {
  args: {
    header: "Header",
    text: "Explain behavior that is not clear from the setting or action name.",
    shortcut: ["⌘", "⌥", "s"],
    link: {
      label: "External",
      url: "https://github.com/keenawa-co/moss",
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const Header: Story = {
  args: {
    header: "Header",
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const Text: Story = {
  args: {
    text: "Explain behavior that is not clear from the setting or action name.",
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const HeaderWithShortcut: Story = {
  args: {
    header: "Header",
    shortcut: ["⌘", "⌥", "s"],
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const WithShortcutAndLink: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
    header: "Settings",
    link: {
      label: "moss",
      url: "https://github.com/keenawa-co/moss",
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const WithArrow: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
    header: "Settings",
    arrow: true,
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const AlwaysOpen: Story = {
  args: {
    header: "Header",
    text: "Explain behavior that is not clear from the setting or action name.",
    shortcut: ["⌘", "⌥", "s"],
    link: {
      label: "External",
      url: "https://github.com/keenawa-co/moss",
    },
    options: {
      root: {
        open: true,
      },
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};
