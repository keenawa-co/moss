import { useState } from "react";

import { Icon } from "@repo/moss-ui";
import { Meta, StoryObj } from "@storybook/react";

import * as RadioGroup from "./RadioGroup";

const meta: Meta = {
  title: "Desktop/RadioGroup",
  component: RadioGroup.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof RadioGroup.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithLabel: Story = {
  render: () => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [value, setValue] = useState("1");
    return (
      <div>
        <RadioGroup.Root className="flex flex-col gap-2">
          <span className="flex gap-2">
            <RadioGroup.Item value="1" id="c1" checked={value === "1"} onClick={() => setValue("1")}>
              <RadioGroup.Indicator>
                <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
              </RadioGroup.Indicator>
            </RadioGroup.Item>
            <label htmlFor="c1">Radio 1</label>
          </span>

          <span className="flex gap-2">
            <RadioGroup.Item value="2" id="c2" checked={value === "2"} onClick={() => setValue("2")}>
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

export const Standalone: Story = {
  render: () => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [value, setValue] = useState("1");

    return (
      <div>
        <RadioGroup.Root className="flex gap-2">
          <RadioGroup.Item value="1" id="c1" checked={value === "1"} onClick={() => setValue("1")}>
            <RadioGroup.Indicator>
              <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
            </RadioGroup.Indicator>
          </RadioGroup.Item>

          <RadioGroup.Item value="2" id="c2" checked={value === "2"} onClick={() => setValue("2")}>
            <RadioGroup.Indicator>
              <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
            </RadioGroup.Indicator>
          </RadioGroup.Item>
        </RadioGroup.Root>
      </div>
    );
  },
};

export const Disabled: Story = {
  render: () => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [value, setValue] = useState("1");
    return (
      <div>
        <RadioGroup.Root className="flex flex-col gap-2" disabled>
          <span className="flex gap-2">
            <RadioGroup.Item value="1" id="c1" checked={value === "1"} onClick={() => setValue("1")}>
              <RadioGroup.Indicator>
                <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
              </RadioGroup.Indicator>
            </RadioGroup.Item>
            <label htmlFor="c1">Radio 1</label>
          </span>

          <span className="flex gap-2">
            <RadioGroup.Item value="2" id="c2" checked={value === "2"} onClick={() => setValue("2")}>
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
