import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import IconHome from "./IconHome";

const meta: Meta<typeof IconHome> = {
  title: 'Icons/IconHome',
  component: IconHome,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs']
};
export default meta;

type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    className: 'w-15 h-15 fill-white',
  },
};

export const Secondary: Story = {
  args: {
    className: 'w-10 h-10 fill-red',
  },
};