import { Meta, StoryObj } from "@storybook/react";

import Switch from "./Switch";

const meta: Meta = {
  title: "Moss-ui/Switch",
  component: Switch,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Switch>;

export default meta;
type Story = StoryObj<typeof meta>;

export const All: Story = {
  render: () => {
    return (
      <div className="flex h-7 flex-col gap-4">
        <Switch />
        <Switch checked />
      </div>
    );
  },
};
