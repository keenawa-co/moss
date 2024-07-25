import type { Meta, StoryObj } from '@storybook/react'
import { fn } from '@storybook/test'
import { HomeIcon } from './HomeIcon'

const meta: Meta<typeof HomeIcon> = {
  title: 'Shared/Icons/IconHome',
  component: HomeIcon,
  parameters: {
    layout: 'centered'
  },
  tags: ['autodocs']
}
export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    className: 'text-stone-500 w-4.5 h-4.5'
  }
}

export const Hover: Story = {
  args: {
    className: 'text-stone-600 w-4.5 h-4.5'
  }
}

export const Active: Story = {
  args: {
    className: 'text-olive-700 w-4.5 h-4.5'
  }
}

export const Disabled: Story = {
  args: {
    className: 'text-stone-500 opacity-50 w-4.5 h-4.5'
  }
}
