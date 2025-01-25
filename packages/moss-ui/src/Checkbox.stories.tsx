import { Meta, StoryObj } from "@storybook/react";

import { Checkbox, Icon } from "./index";

const meta: Meta = {
  title: "Moss-ui/Checkbox",
  component: Checkbox.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Checkbox.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const All: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-4">
        <div className="flex gap-2">
          <Checkbox.Root id="c1">
            <Checkbox.Indicator>
              <Icon icon="DropdownMenuCheckboxIndicator" className="text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c1">Checkbox 1</label>
        </div>
        <div className="flex gap-2">
          <Checkbox.Root id="c2">
            <Checkbox.Indicator>
              <Icon icon="DropdownMenuCheckboxIndicator" className="text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c2">Checkbox 2</label>
        </div>
        <div className="flex gap-2">
          <Checkbox.Root id="c3">
            <Checkbox.Indicator>
              <Icon icon="DropdownMenuCheckboxIndicator" className="text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c3">Checkbox 3</label>
        </div>
      </div>
    );
  },
};
