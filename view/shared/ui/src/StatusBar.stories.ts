import type { Meta, StoryObj } from '@storybook/react'
import StatusBar from './StatusBar'

const meta = {
  title: 'StatusBar',
  component: StatusBar,
  tags: ['autodocs'],
  parameters: {
    layout: 'fullscreen'
  },
  args: {}
} satisfies Meta<typeof StatusBar>

export default meta
type Story = StoryObj<typeof meta>

export const WithBranch: Story = {
  args: {
    branch: 'MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri'
  }
}

export const NoBranch: Story = {}
