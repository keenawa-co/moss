import { Icon } from "@repo/moss-ui";
import { Meta, StoryObj } from "@storybook/react";

import Button from "./Button";

const variants = ["solid", "outlined", "soft", "ghost"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const intents = {
  primary: {
    bg: "#0073ca",
    bgHover: "#0c92eb",
    border: "#0073ca",
    text: "white",
    ring: "#b9e0fe",
  },
  warning: {
    bg: "#d1bf00",
    bgHover: "#ffff00",
    border: "#d1bf00",
    text: "white",
    ring: "#eeff86",
  },
  success: {
    bg: "#53b800",
    bgHover: "#6ee600",
    border: "#53b800",
    text: "white",
    ring: "#d0ff90",
  },
  danger: {
    bg: "#ff0000",
    bgHover: "#ff5757",
    border: "#ff0000",
    text: "white",
    ring: "#ffc0c0",
  },
  neutral: {
    bg: "#969696",
    bgHover: "#aaaaaa",
    border: "#969696",
    text: "white",
    ring: "#e3e3e3",
  },
} as const;

const meta: Meta = {
  title: "Desktop/Button",
  component: Button.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
  decorators: [
    (Story) => (
      <div
        style={
          {
            "--color-button-primary-bg": intents.primary.bg,
            "--color-button-primary-bg-hover": intents.primary.bgHover,
            "--color-button-primary-border": intents.primary.border,
            "--color-button-primary-text": intents.primary.text,
            "--color-button-primary-ring": intents.primary.ring,

            "--color-button-warning-bg": intents.warning.bg,
            "--color-button-warning-bg-hover": intents.warning.bgHover,
            "--color-button-warning-border": intents.warning.border,
            "--color-button-warning-text": intents.warning.text,
            "--color-button-warning-ring": intents.warning.ring,

            "--color-button-success-bg": intents.success.bg,
            "--color-button-success-bg-hover": intents.success.bgHover,
            "--color-button-success-border": intents.success.border,
            "--color-button-success-text": intents.success.text,
            "--color-button-success-ring": intents.success.ring,

            "--color-button-danger-bg": intents.danger.bg,
            "--color-button-danger-bg-hover": intents.danger.bgHover,
            "--color-button-danger-border": intents.danger.border,
            "--color-button-danger-text": intents.danger.text,
            "--color-button-danger-ring": intents.danger.ring,

            "--color-button-neutral-bg": intents.neutral.bg,
            "--color-button-neutral-bg-hover": intents.neutral.bgHover,
            "--color-button-neutral-border": intents.neutral.border,
            "--color-button-neutral-text": intents.neutral.text,
            "--color-button-neutral-ring": intents.neutral.ring,
          } as React.CSSProperties
        }
      >
        <Story />
      </div>
    ),
  ],
  args: {
    loading: false,
    disabled: false,
    href: undefined,
    intent: "primary",
    variant: "solid",
    size: "md",
  },
} satisfies Meta<typeof Button.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  render: (args) => {
    return (
      <Button.Root {...args}>
        <Button.Label>Button</Button.Label>
      </Button.Root>
    );
  },
};

export const IntentsAndVariants: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          <th />
          {variants.map((variant) => {
            return <th className="text-left capitalize">{variant}</th>;
          })}
        </tr>
        {(Object.keys(intents) as Array<keyof typeof intents>).map((intent) => {
          return (
            <tr key={intent}>
              <th className="text-left capitalize">{intent}</th>
              {variants.map((variant) => {
                return (
                  <td key={variant}>
                    <Button.Root intent={intent} variant={variant}>
                      <Button.Label>Button</Button.Label>
                    </Button.Root>
                  </td>
                );
              })}
            </tr>
          );
        })}
      </table>
    );
  },
};

export const Intents: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          {Object.keys(intents).map((intent) => {
            return <th className="text-left capitalize">{intent}</th>;
          })}
        </tr>
        <tr>
          {(Object.keys(intents) as Array<keyof typeof intents>).map((intent) => {
            return (
              <td key={intent}>
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
  args: {
    disabled: true,
  },
  render: (args) => {
    return (
      <Button.Root {...args}>
        <Button.Label>Button</Button.Label>
      </Button.Root>
    );
  },
};

export const Loading: Story = {
  args: {
    loading: true,
  },
  render: (args) => {
    return (
      <Button.Root {...args}>
        <Button.Label>Button</Button.Label>
      </Button.Root>
    );
  },
};

export const IconWithLabel: Story = {
  render: (args) => {
    return (
      <Button.Root {...args} className="flex gap-2">
        <Button.Label>Button</Button.Label>
        <Icon icon="ArrowRight" />
      </Button.Root>
    );
  },
};

export const IconOnly: Story = {
  render: (args) => {
    return (
      <Button.Root {...args}>
        <Icon icon="ArrowRight" />
      </Button.Root>
    );
  },
};
