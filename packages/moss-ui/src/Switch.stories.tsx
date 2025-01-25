import { Meta, StoryObj } from "@storybook/react";

import { Switch } from "./index";

const meta: Meta = {
  title: "Moss-ui/Switch",
  component: Switch.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Switch.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const All: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-4">
        <Switch.Root className="">
          <Switch.Thumb />
        </Switch.Root>

        <Switch.Root checked>
          <Switch.Thumb />
        </Switch.Root>
      </div>
    );
  },
};
