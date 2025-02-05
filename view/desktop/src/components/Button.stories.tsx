import { Icon } from "@repo/moss-ui";
import { Meta, StoryObj } from "@storybook/react";

import Button from "./Button";

const variants = ["solid", "outlined", "soft", "ghost"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const colors = {
  primary: {
    solid: {
      bg: "linear-gradient(in oklab, rgb(59, 130, 246) 0%, rgb(37, 99, 235) 100%)",
      border: "#1d4ed8",
      text: "white",
    },
    outlined: {
      bg: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(239, 246, 255) 100%)",
      border: "#bfdbfe",
      text: "#1e40af",
    },
    soft: {
      bg: "#dbeafe",
      border: "#dbeafe",
      text: "#1d4ed8",
    },
    ghost: {
      bg: "#dbeafe",
      border: "#dbeafe",
      text: "#1d4ed8",
    },
  },
  warning: {
    solid: {
      bg: "linear-gradient(rgb(250, 204, 21), rgb(234, 179, 8))",
      border: "#ca8a04",
      text: "#422006",
    },
    outlined: {
      bg: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(254, 252, 232) 100%)",
      border: "#fef08a",
      text: "#854d0e",
    },
    soft: {
      bg: "rgb(254, 249, 195)",
      border: "rgb(254, 249, 195)",
      text: "#a16207",
    },
    ghost: {
      bg: "rgb(254, 249, 195)",
      border: "rgb(254, 249, 195)",
      text: "#a16207",
    },
  },
  success: {
    solid: {
      bg: "linear-gradient(rgb(34, 197, 94), rgb(22, 163, 74))",
      border: "#15803d",
      text: "white",
    },
    outlined: {
      bg: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(220, 252, 231) 100%)",
      border: "#bbf7d0",
      text: "rgb(22, 101, 52)",
    },
    soft: {
      bg: "rgb(220, 252, 231)",
      border: "rgb(220, 252, 231)",
      text: "rgb(21, 128, 61)",
    },
    ghost: {
      bg: "rgb(220, 252, 231)",
      border: "rgb(220, 252, 231)",
      text: "rgb(21, 128, 61)",
    },
  },
  danger: {
    solid: {
      bg: "linear-gradient(rgb(239, 68, 68), rgb(220, 38, 38))",
      border: "#b91c1c",
      text: "white",
    },
    outlined: {
      bg: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(254, 242, 242) 100%)",
      border: "#fdcfcf",
      text: "rgb(153, 27, 27)",
    },
    soft: {
      bg: "rgb(254, 226, 226)",
      border: "rgb(254, 226, 226)",
      text: "rgb(185, 28, 28)",
    },
    ghost: {
      bg: "rgb(254, 226, 226)",
      border: "rgb(254, 226, 226)",
      text: "rgb(185, 28, 28)",
    },
  },
  neutral: {
    solid: {
      bg: "linear-gradient(rgb(113, 113, 122), rgb(82, 82, 91))",
      border: "#3f3f46",
      text: "white",
    },
    outlined: {
      bg: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(250, 250, 250) 100%)",
      border: "#e4e4e7",
      text: "rgb(39, 39, 42)",
    },
    soft: {
      bg: "rgb(244, 244, 245)",
      border: "rgb(244, 244, 245)",
      text: "rgb(39, 39, 42)",
    },
    ghost: {
      bg: "rgb(244, 244, 245)",
      border: "rgb(244, 244, 245)",
      text: "rgb(39, 39, 42)",
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
