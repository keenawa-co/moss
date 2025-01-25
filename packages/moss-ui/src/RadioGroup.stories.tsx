import { Meta, StoryObj } from "@storybook/react";

import { Icon, RadioGroup } from "./index";

const meta: Meta = {
  title: "Moss-ui/RadioGroup",
  component: RadioGroup.RadioGroupRoot,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof RadioGroup.RadioGroupRoot>;

export default meta;
type Story = StoryObj<typeof meta>;

export const All: Story = {
  render: () => {
    return (
      <div>
        <RadioGroup.RadioGroupRoot id="c1" className="flex flex-col gap-2">
          <span className="flex gap-2">
            <RadioGroup.RadioGroupItem value="1" className="size-4">
              <RadioGroup.RadioGroupIndicator>
                <Icon icon="ShortcutItemCheck" className="text-black" />
              </RadioGroup.RadioGroupIndicator>
            </RadioGroup.RadioGroupItem>
            <div>Radio 1</div>
          </span>

          <span className="flex gap-2">
            <RadioGroup.RadioGroupItem value="2" className="size-4">
              <RadioGroup.RadioGroupIndicator>
                <Icon icon="ShortcutItemCheck" className="text-black" />
              </RadioGroup.RadioGroupIndicator>
            </RadioGroup.RadioGroupItem>

            <div>Radio 2</div>
          </span>
        </RadioGroup.RadioGroupRoot>
      </div>
    );
  },
};
