import type { Meta, StoryObj } from "@storybook/react";
import Tooltip from "./Tooltip";

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

const TooltipTemplate = (args: any) => <Tooltip {...args}>Hover me!</Tooltip>;

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
export const Shortcut: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
  },
  render: (args) => <TooltipTemplate {...args} />,
};
export const Link: Story = {
  args: {
    link: {
      label: "External",
      url: "https://github.com/keenawa-co/moss",
    },
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

export const ALotOfText: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
    text: 'Not a lorem ipsum text, because spell checker marks words from lorem ipsum text as "problems"',
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const NoArrow: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
    header: "Settings",
    noArrow: true,
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const AlignTooltipBottom: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
    header: "Settings",
    options: {
      content: {
        side: "bottom",
      },
    },
  },
  render: (args) => <TooltipTemplate {...args} />,
};

export const AlignTooltipStart: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
    header: "Settings and some other stuff",
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
    shortcut: ["⌘", "⌥", "s"],
    header: "Settings",
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

export const IconTrigger: Story = {
  args: {
    shortcut: ["⌘", "⌥", "s"],
    header: "Settings",
    options: {
      root: {},
    },
  },
  render: (args) => (
    <Tooltip {...args}>
      <svg
        className="flex items-center fill-current"
        viewBox="0 0 16 18"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        width={64}
        height={64}
      >
        <path
          fill-rule="evenodd"
          clip-rule="evenodd"
          d="M4.21325 5.41151C4.45586 5.12846 4.88199 5.09569 5.16503 5.3383C5.44808 5.58091 5.48085 6.00703 5.23824 6.29008L2.91477 9.00079L5.23824 11.7115C5.48085 11.9946 5.44808 12.4207 5.16503 12.6633C4.88199 12.9059 4.45586 12.8731 4.21325 12.5901L1.13672 9.00079L4.21325 5.41151ZM13.7883 5.41151C13.5457 5.12846 13.1195 5.09569 12.8365 5.3383C12.5534 5.58091 12.5207 6.00703 12.7633 6.29008L15.0867 9.00079L12.7633 11.7115C12.5207 11.9946 12.5534 12.4207 12.8365 12.6633C13.1195 12.9059 13.5457 12.8731 13.7883 12.5901L16.8648 9.00079L13.7883 5.41151Z"
        />
        <path
          fill-rule="evenodd"
          clip-rule="evenodd"
          d="M10.7446 2.27177C11.1054 2.36532 11.3221 2.7337 11.2286 3.09456L8.07856 15.2446C7.98501 15.6054 7.61663 15.8221 7.25576 15.7286C6.8949 15.635 6.67821 15.2666 6.77177 14.9058L9.92177 2.75576C10.0153 2.3949 10.3837 2.17821 10.7446 2.27177Z"
        />
      </svg>
    </Tooltip>
  ),
};

export const TriggerAsChild: Story = {
  args: {
    text: "Generally when you pass text to inside tooltip tag it will wrap it in button. If you don't need this wrapper you set asChild prop to true",
    asChild: true,
  },
  render: (args) => (
    <Tooltip {...args}>
      <span>Hover me!</span>
    </Tooltip>
  ),
};
