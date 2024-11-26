import type { Meta, StoryObj } from "@storybook/react";
import "@/assets/index.css";
import StatusBar from "@/components/StatusBar";

const meta = {
  title: "desktop/StatusBar",
  component: StatusBar,
  tags: ["autodocs"],
  parameters: {
    layout: "fullscreen",
  },
  args: {},
} satisfies Meta<typeof StatusBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
