import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import IconHome from "./IconHome";

const meta = {
  title: 'IconHome',
  component: IconHome,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs']

} satisfies Meta<typeof IconHome>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    className: 'text-neutral-500',
  },
};

export const Secondary: Story = {
  args: {
    className: 'text-red-500',
  },
};