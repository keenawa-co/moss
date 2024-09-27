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

export const AllVariants: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl",
  },
  render: (args) => {
    return (
      <div className="flex w-full flex-col gap-6 p-16">
        <h2 className="text-3xl font-semibold">All variants</h2>

        <div className="grid grid-cols-4 justify-items-center gap-4">
          <div>Default</div>
          <div>Fill</div>
          <div>Stroke</div>
          <div>Without default color</div>
          <Icon icon={args.icon} className="text-6xl" />
          <Icon icon={args.icon} className="text-6xl text-green-400" />
          <Icon icon={args.icon} className="stroke-green-400 stroke-1 text-6xl" viewBox="-1 -1 18 17" />
          <Icon icon="NewProject" className="text-6xl" />
        </div>
      </div>
    );
  },
};

export const Default: Story = {
  args: {
    icon: "Home1",
    className: "text-6xl",
  },
};

export const Stroke: Story = {
  args: {
    icon: "Goals",
    className: "text-6xl stroke-1 stroke-red-500",
    viewBox: "0 0 20 20",
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
