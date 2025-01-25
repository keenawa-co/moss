import { Meta, StoryObj } from "@storybook/react";

import { Checkbox, Icon } from "./index";

const meta: Meta = {
  title: "Moss-ui/Checkbox",
  component: Checkbox.CheckboxRoot,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Checkbox.CheckboxRoot>;

export default meta;
type Story = StoryObj<typeof meta>;

export const All: Story = {
  render: () => {
    return (
      <div className="flex h-7 flex-col gap-4">
        <Checkbox.CheckboxRoot id="c1" className="size-4">
          <Checkbox.CheckboxIndicator>
            <Icon icon="ArrowRight" className="text-black" />
          </Checkbox.CheckboxIndicator>
        </Checkbox.CheckboxRoot>
      </div>
    );
  },
};
