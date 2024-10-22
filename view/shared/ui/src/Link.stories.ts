import { Meta, StoryObj } from "@storybook/react";
import { Link } from "./index";

const meta: Meta = {
  title: "Shared/Link",
  component: Link,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Link>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    label: "Primary",
    url: "https://www.google.com",
  },
};

export const WithIcon: Story = {
  args: {
    label: "Primary",
    url: "https://www.google.com",
    withIcon: true,
  },
};

export const Secondary: Story = {
  args: {
    label: "Secondary",
    url: "https://www.google.com",
    withIcon: true,
  },
};

export const Disabled: Story = {
  args: {
    label: "Disabled",
    url: "https://www.google.com",
    type: "disabled",
  },
};
