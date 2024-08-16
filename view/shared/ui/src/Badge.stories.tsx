import type { Meta, StoryObj } from "@storybook/react";
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

export const AllVariants: Story = {
  args: {},
  render: (args) => {
    return (
      <div className="w-full p-16 flex flex-col gap-6">
        <h2 className="text-3xl font-semibold">All variants</h2>

        <table className="w-64">
          <tr>
            <th align="left">Full</th>
            <th align="left">Compact</th>
          </tr>

          <tr>
            <td align="left">
              <Badge {...args} value="Text" />
            </td>
            <td align="left">
              <Badge {...args} style="compact" />
            </td>
          </tr>
        </table>
      </div>
    );
  },
};

export const Default: Story = {
  args: {
    value: "Text",
  },
};

export const Compact: Story = {
  args: {
    style: "compact",
  },
};
