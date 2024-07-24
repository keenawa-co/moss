import type { Meta, StoryObj } from '../../../storybook/node_modules/@storybook/react';
import '../assets/index.css';
import StatusBar from './StatusBar';

const meta = {
  title: 'desktop/ui/StatusBar',
  component: StatusBar,
  tags: ['autodocs'],
  parameters: {
    layout: 'fullscreen',
  },
  args: {},
} satisfies Meta<typeof StatusBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithBranch: Story = {
  args: {
    branch: 'MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri',
  },
};

export const NoBranch: Story = {};
