import { Meta, StoryObj } from "@storybook/react";

import * as Switch from "./Switch";

const meta: Meta = {
  title: "Desktop/Switch",
  component: Switch.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Switch.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithLabel: Story = {
  render: () => {
    return (
      <div className="flex gap-4">
        <Switch.Root className="">
          <Switch.Thumb />
        </Switch.Root>
        <div>Label</div>
      </div>
    );
  },
};

export const Standalone: Story = {
  render: () => {
    return (
      <div className="flex">
        <Switch.Root>
          <Switch.Thumb />
        </Switch.Root>
      </div>
    );
  },
};
