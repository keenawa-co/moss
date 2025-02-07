import { Meta, StoryObj } from "@storybook/react";

import Button from "./Button";
import Icon from "./Icon";

const variants = ["solid", "outlined", "soft", "ghost"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const colors = {
  primary: {
    solid: {
      background: "linear-gradient(in oklab, rgb(59, 130, 246) 0%, rgb(37, 99, 235) 100%)",
      border: null,
      text: "white",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#1d4ed8 0px 0px 0px 1px",
    },
    outlined: {
      background: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(239, 246, 255) 100%)",
      border: null,
      text: "#1e40af",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#bfdbfe 0px 0px 0px 1px",
    },
    soft: {
      background: "#dbeafe",
      border: null,
      text: "#1d4ed8",
      boxShadow: "#dbeafe 0px 0px 0px 1px",
    },
    ghost: {
      background: "#dbeafe",
      border: "#dbeafe",
      text: "#1d4ed8",
      boxShadow: null,
    },
  },
  warning: {
    solid: {
      background: "linear-gradient(rgb(250, 204, 21), rgb(234, 179, 8))",
      border: null,
      text: "#422006",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#ca8a04 0px 0px 0px 1px",
    },
    outlined: {
      background: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(254, 252, 232) 100%)",
      border: null,
      text: "#854d0e",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#fef08a 0px 0px 0px 1px",
    },
    soft: {
      background: "rgb(254, 249, 195)",
      border: null,
      text: "#a16207",
      boxShadow: "rgb(254,249,195) 0px 0px 0px 1px",
    },
    ghost: {
      background: "rgb(254, 249, 195)",
      border: "rgb(254, 249, 195)",
      text: "#a16207",
      boxShadow: null,
    },
  },
  success: {
    solid: {
      background: "linear-gradient(rgb(34, 197, 94), rgb(22, 163, 74))",
      border: null,
      text: "white",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#15803d 0px 0px 0px 1px",
    },
    outlined: {
      background: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(220, 252, 231) 100%)",
      border: null,
      text: "rgb(22, 101, 52)",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#bbf7d0 0px 0px 0px 1px",
    },
    soft: {
      background: "rgb(220, 252, 231)",
      border: "rgb(220, 252, 231)",
      text: "rgb(21, 128, 61)",
      boxShadow: "rgb(220, 252, 231) 0px 0px 0px 1px",
    },
    ghost: {
      background: "rgb(220, 252, 231)",
      border: "rgb(220, 252, 231)",
      text: "rgb(21, 128, 61)",
      boxShadow: null,
    },
  },
  danger: {
    solid: {
      background: "linear-gradient(rgb(239, 68, 68), rgb(220, 38, 38))",
      border: null,
      text: "white",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#b91c1c 0px 0px 0px 1px",
    },
    outlined: {
      background: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(254, 242, 242) 100%)",
      border: null,
      text: "rgb(153, 27, 27)",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#fdcfcf 0px 0px 0px 1px",
    },
    soft: {
      background: "rgb(254, 226, 226)",
      border: null,
      text: "rgb(185, 28, 28)",
      boxShadow: "rgb(254, 226, 226) 0px 0px 0px 1px",
    },
    ghost: {
      background: "rgb(254, 226, 226)",
      border: "rgb(254, 226, 226)",
      text: "rgb(185, 28, 28)",
      boxShadow: null,
    },
  },
  neutral: {
    solid: {
      background: "linear-gradient(rgb(113, 113, 122), rgb(82, 82, 91))",
      border: null,
      text: "white",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#3f3f46 0px 0px 0px 1px",
    },
    outlined: {
      background: "radial-gradient(76% 151% at 52% -52%, rgba(255, 255, 255, 0.6) 0%, rgb(250, 250, 250) 100%)",
      border: null,
      text: "rgb(39, 39, 42)",
      boxShadow: "rgba(255,255,255,0.25) 0px 1px 0px 0px inset,#e4e4e7 0px 0px 0px 1px",
    },
    soft: {
      background: "rgb(244, 244, 245)",
      border: null,
      text: "rgb(39, 39, 42)",
      boxShadow: "rgb(244, 244, 245) 0px 0px 0px 1px",
    },
    ghost: {
      background: "rgb(244, 244, 245)",
      border: "rgb(244, 244, 245)",
      text: "rgb(39, 39, 42)",
      boxShadow: null,
    },
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
      <div className="text-(--moss-primary)">
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
      <table className="border-separate border-spacing-2">
        <tr>
          {sizes.map((size) => {
            return (
              <td key={size}>
                <Button.Root {...args} size={size}>
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
