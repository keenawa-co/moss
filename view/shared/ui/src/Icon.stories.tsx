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
    className: "text-6xl",
  },
};

export const Stroke: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl  stroke-1 stroke-red-500",
    viewBox: "-1 0 18 17",
  },
};

export const Fill: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl text-green-300",
  },
};

export const WithoutDefaultColor: Story = {
  args: {
    icon: "NewProject",
    className: "text-6xl text-red-300",
  },
};
