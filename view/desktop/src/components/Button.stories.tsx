import { Icon } from "@repo/moss-ui";
import { Meta, StoryObj } from "@storybook/react";

import Button from "./Button";

const variants = ["solid", "outlined", "soft", "ghost"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const colors = {
  primary: {
    solid: {
      bg: "#2563eb",
      border: "#0073ca",
      text: "white",
    },
    outlined: {
      bg: "#141b29",
      border: "rgb(29, 50, 87)",
      text: "rgb(147, 197, 253)",
    },
    soft: {
      bg: "#141b29",
      border: "#141b29",
      text: "rgb(147, 197, 253)",
    },
    ghost: {
      bg: "#141b29",
      border: "#141b29",
      text: "rgb(147, 197, 253)",
    },
  },
  warning: {
    solid: {
      bg: "#eab50e",
      border: "#eab50e",
      text: "#422006",
    },
    outlined: {
      bg: "#262010",
      border: "#51400e",
      text: "#fde047",
    },
    soft: {
      bg: "#262010",
      border: "#262010",
      text: "#fde047",
    },
    ghost: {
      bg: "#262010",
      border: "#262010",
      text: "#fde047",
    },
  },
  success: {
    solid: {
      bg: "#16a34a",
      border: "#16a34a",
      text: "white",
    },
    outlined: {
      bg: "#112219",
      border: "#144628",
      text: "#86efac",
    },
    soft: {
      bg: "#112219",
      border: "#112219",
      text: "#86efac",
    },
    ghost: {
      bg: "#112219",
      border: "#112219",
      text: "#86efac",
    },
  },
  danger: {
    solid: {
      bg: "#dc2626",
      border: "#dc2626",
      text: "white",
    },
    outlined: {
      bg: "#1c1214",
      border: "#521e20",
      text: "#fca5a5",
    },
    soft: {
      bg: "#1c1214",
      border: "#1c1214",
      text: "#fca5a5",
    },
    ghost: {
      bg: "#1c1214",
      border: "#1c1214",
      text: "#fca5a5",
    },
  },
  neutral: {
    solid: {
      bg: "#52525b",
      border: "#52525b",
      text: "white",
    },
    outlined: {
      bg: "#19191c",
      border: "#2c2c31",
      text: "white",
    },
    soft: {
      bg: "#19191c",
      border: "#19191c",
      text: "white",
    },
    ghost: {
      bg: "#19191c",
      border: "#19191c",
      text: "white",
    },
  },
} as const;

const convertColorsToCSSVars = (
  colors: Record<string, Record<string, Record<string, string>>>
): Record<string, string> => {
  const cssVars: Record<string, string> = {};

  for (const [category, variants] of Object.entries(colors)) {
    for (const [variant, properties] of Object.entries(variants)) {
      for (const [prop, value] of Object.entries(properties)) {
        const cssVarName = `--color-button-${category}-${variant}-${prop}`.replace(/bg/g, "background");
        cssVars[cssVarName] = value;
      }
    }
  }

  return cssVars;
};

const meta: Meta = {
  title: "Desktop/Button",
  component: Button.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
  decorators: [
    (Story) => (
      <div style={convertColorsToCSSVars(colors)} className="text-(--moss-primary)">
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

export const Intents: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          <th />
          {variants.map((variant) => {
            return <th className="text-left capitalize">{variant}</th>;
          })}
        </tr>
        {(Object.keys(colors) as Array<keyof typeof colors>).map((intent) => {
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

export const Sizes: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          <th />
          {sizes.map((size) => {
            return <th className="text-left capitalize">{size}</th>;
          })}
        </tr>
        {(Object.keys(colors) as Array<keyof typeof colors>).map((intent) => {
          return (
            <tr key={intent} className="align-bottom">
              <th className="text-left capitalize">{intent}</th>
              {sizes.map((size) => {
                return (
                  <td key={size}>
                    <Button.Root intent={intent} size={size}>
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

export const WithIcons: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        <tr>
          <td>
            <Button.Root className="flex gap-2">
              <Icon icon="ArrowRight" />
              <Button.Label>Label</Button.Label>
            </Button.Root>
          </td>
          <td>
            <Button.Root>
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
          <td>
            <Button.Root className="flex gap-2">
              <Button.Label>Label</Button.Label>
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
        </tr>
        <tr>
          <td>
            <Button.Root variant="outlined" className="flex gap-2">
              <Icon icon="ArrowRight" />
              <Button.Label>Label</Button.Label>
            </Button.Root>
          </td>
          <td>
            <Button.Root variant="outlined">
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
          <td>
            <Button.Root variant="outlined" className="flex gap-2">
              <Button.Label>Label</Button.Label>
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
        </tr>
        <tr>
          <td>
            <Button.Root variant="soft" className="flex gap-2">
              <Icon icon="ArrowRight" />
              <Button.Label>Label</Button.Label>
            </Button.Root>
          </td>
          <td>
            <Button.Root variant="soft">
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
          <td>
            <Button.Root variant="soft" className="flex gap-2">
              <Button.Label>Label</Button.Label>
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
        </tr>
        <tr>
          <td>
            <Button.Root variant="ghost" className="flex gap-2">
              <Icon icon="ArrowRight" />
              <Button.Label>Label</Button.Label>
            </Button.Root>
          </td>
          <td>
            <Button.Root variant="ghost">
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
          <td>
            <Button.Root variant="ghost" className="flex gap-2">
              <Button.Label>Label</Button.Label>
              <Icon icon="ArrowRight" />
            </Button.Root>
          </td>
        </tr>
      </table>
    );
  },
};
