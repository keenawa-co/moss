import { Icon } from "@repo/moss-ui";
import { Meta, StoryObj } from "@storybook/react";

import Button from "./Button";

const variants = ["solid", "outlined", "soft", "ghost"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const intents = ["primary", "warning", "success", "danger", "neutral"] as const;

const meta: Meta = {
  title: "Desktop/Button",
  component: Button.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Button.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Intents: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          {intents.map((intent) => {
            return <th className="text-left capitalize">{intent}</th>;
          })}
        </tr>
        <tr>
          {intents.map((intent) => {
            return (
              <td>
                <Button.Root intent={intent}>
                  <Button.Label>Button</Button.Label>
                </Button.Root>
              </td>
            );
          })}
        </tr>
      </table>
    );
  },
};

export const Variants: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          {variants.map((variant) => {
            return <th className="text-left capitalize">{variant}</th>;
          })}
        </tr>
        <tr>
          {variants.map((variant) => {
            return (
              <td>
                <Button.Root variant={variant}>
                  <Button.Label>Button</Button.Label>
                </Button.Root>
              </td>
            );
          })}
        </tr>
      </table>
    );
  },
};

export const Sizes: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          {sizes.map((size) => {
            return <th className="text-left capitalize">{size}</th>;
          })}
        </tr>
        <tr>
          {sizes.map((size) => {
            return (
              <td>
                <Button.Root size={size}>
                  <Button.Label>Button</Button.Label>
                </Button.Root>
              </td>
            );
          })}
        </tr>
      </table>
    );
  },
};

export const Disabled: Story = {
  render: () => {
    return (
      <Button.Root disabled>
        <Button.Label>Button</Button.Label>
      </Button.Root>
    );
  },
};

export const Loading: Story = {
  render: () => {
    return (
      <Button.Root loading>
        <Button.Label>Button</Button.Label>
      </Button.Root>
    );
  },
};

export const IconWithLabel: Story = {
  render: () => {
    return (
      <Button.Root className="flex gap-2">
        <Button.Label>Button</Button.Label>
        <Icon icon="ArrowRight" />
      </Button.Root>
    );
  },
};

export const IconOnly: Story = {
  render: () => {
    return (
      <Button.Root>
        <Icon icon="ArrowRight" />
      </Button.Root>
    );
  },
};
