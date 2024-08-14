import type { Meta, StoryObj } from "@storybook/react";
import { Icon, Icons } from "./Icon";
import * as icons from "../../icons/build";

const iconOptions = Object.keys(icons) as Icons[];

const meta = {
  title: "Shared/Icon",
  component: Icon,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
  args: {
    className: "text-6xl",
  },
  argTypes: {
    icon: { control: { type: "select" }, options: iconOptions },
  },
} satisfies Meta<typeof Icon>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl text-zinc-300",
  },
};

export const IconWithoutChangeableColor: Story = {
  args: {
    icon: "NewProject",
    className: "text-6xl text-red-300",
  },
};
