import type { Meta, StoryObj } from "@storybook/react";
import { expect, userEvent, within } from "@storybook/test";

import { Badge } from "./Badge";

const meta: Meta<typeof Badge> = {
  title: "Shared/Badge",
  component: Badge,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
    design: [
      {
        name: "Badge",
        type: "figma",
        url: "https://www.figma.com/design/acKJvhO9lMOv9wVObplm0H/M?node-id=795-1432&t=XoICfYcPXgqBcA1B-4",
      },
    ],
  },
  args: {
    color: [
      [80, 48, 229, 100],
      [225, 221, 239, 100],
    ],
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  tags: ["stable"],
  args: {
    value: "Text",
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    expect(canvas.getByText("Text")).toBeInTheDocument();
  },
};

export const Compact: Story = {
  args: {
    style: "compact",
  },
};
