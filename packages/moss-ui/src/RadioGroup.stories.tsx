import { Meta, StoryObj } from "@storybook/react";

import { Icon, RadioGroup } from "./index";

const meta: Meta = {
  title: "Moss-ui/RadioGroup",
  component: RadioGroup.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof RadioGroup.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const All: Story = {
  render: () => {
    return (
      <div>
        <RadioGroup.Root className="RadioGroup.Root flex flex-col gap-2">
          <span className="flex gap-2">
            <RadioGroup.Item value="1" id="c1">
              <RadioGroup.Indicator>
                <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
              </RadioGroup.Indicator>
            </RadioGroup.Item>
            <label htmlFor="c1">Radio 1</label>
          </span>

          <span className="flex gap-2">
            <RadioGroup.Item value="2" id="c2">
              <RadioGroup.Indicator>
                <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
              </RadioGroup.Indicator>
            </RadioGroup.Item>

            <label htmlFor="c2">Radio 2</label>
          </span>
        </RadioGroup.Root>
      </div>
    );
  },
};
