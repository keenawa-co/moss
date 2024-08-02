import type { Meta, StoryObj } from "@storybook/react";
import TooltipDemo from "./TooltipDemo";
import { Button } from "./Button";
import { useState } from "react";

const meta = {
  title: "Shared/TooltipDemo",
  component: TooltipDemo,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  argTypes: {},
  args: {},
} satisfies Meta<typeof TooltipDemo>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithShortcut: Story = {
  args: {
    shortcut: ["⌘", "s"],
    label: "Settings",
  },
  render: (args) => {
    const [open, setOpen] = useState(false);

    return (
      <TooltipDemo shortcut={args.shortcut} label={args.label} open={open} onOpenChange={setOpen}>
        <button
          onClick={() => {
            setOpen(!open);
          }}
        >
          Tooltip on click
        </button>
      </TooltipDemo>
    );
  },
};

export const NoShortcut: Story = {
  args: {
    label: "Settings",
  },
  render: (args) => {
    const [open, setOpen] = useState(false);

    return (
      <TooltipDemo shortcut={args.shortcut} label={args.label} open={open} onOpenChange={setOpen}>
        <button
          onClick={() => {
            setOpen(!open);
          }}
        >
          Tooltip on click
        </button>
      </TooltipDemo>
    );
  },
};
