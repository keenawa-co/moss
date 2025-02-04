import { useState } from "react";

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
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [value, setValue] = useState(true);
    return (
      <div className="flex flex-col gap-4">
        <div className="flex gap-4">
          <Switch.Root id="test-switch">
            <Switch.Thumb />
          </Switch.Root>
          <label htmlFor="test-switch">Label</label>
        </div>
        <div className="flex gap-4">
          <Switch.Root onClick={() => setValue(!value)} checked={value}>
            <Switch.Thumb />
          </Switch.Root>
          <div>Label</div>
        </div>
      </div>
    );
  },
};

export const Standalone: Story = {
  render: () => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [value, setValue] = useState(true);
    return (
      <div className="flex flex-col gap-4">
        <div className="flex gap-4">
          <Switch.Root>
            <Switch.Thumb />
          </Switch.Root>
        </div>
        <div className="flex gap-4">
          <Switch.Root onClick={() => setValue(!value)} checked={value}>
            <Switch.Thumb />
          </Switch.Root>
        </div>
      </div>
    );
  },
};

export const Disabled: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-4">
        <div className="flex gap-4">
          <Switch.Root disabled>
            <Switch.Thumb />
          </Switch.Root>
          <div>Label</div>
        </div>
        <div className="flex gap-4">
          <Switch.Root disabled checked>
            <Switch.Thumb />
          </Switch.Root>
          <div>Label</div>
        </div>
      </div>
    );
  },
};
