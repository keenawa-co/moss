import { Meta, StoryObj } from "@storybook/react";

import { Button, Icon } from "./index";
import { ButtonStyleProps } from "./types";

const meta: Meta = {
  title: "Moss-ui/Button",
  component: Button.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Button.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

const variants = ["solid", "outlined", "soft", "ghost"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const colors = [
  {
    background: {
      default: "#0073ca",
      hover: "#0c92eb",
    },
    borderColor: {
      default: "#0073ca",
    },
    color: {
      default: "--moss-windowsCloseButton-background",
    },
    ring: "#b9e0fe",
  },
  {
    background: {
      default: "#d1bf00",
      hover: "#ffff00",
    },
    borderColor: {
      default: "#d1bf00",
    },
    color: {
      default: "white",
    },

    ring: "#eeff86",
  },
  {
    background: {
      default: "#53b800",
      hover: "#6ee600",
    },
    borderColor: {
      default: "#53b800",
    },
    color: {
      default: "white",
    },
    ring: "#d0ff90",
  },
  {
    background: {
      default: "#ff0000",
      hover: "#ff5757",
    },
    borderColor: {
      default: "#ff0000",
    },
    color: {
      default: "white",
    },
    ring: "#ffc0c0",
  },
  {
    background: {
      default: "#969696",
      hover: "#aaaaaa",
    },
    borderColor: {
      default: "#969696",
    },
    color: {
      default: "white",
    },
    ring: "#e3e3e3",
  },
] as ButtonStyleProps[];

export const All: Story = {
  render: () => {
    return (
      <div>
        <h2>Intents</h2>
        <table className="border-separate border-spacing-2">
          <tbody>
            <tr>
              <th></th>
              {variants.map((variant) => {
                return <th className="text-left capitalize">{variant}</th>;
              })}
            </tr>

            {colors.map((color, i) => {
              return (
                <tr>
                  <th className="text-left capitalize">{i}</th>
                  {variants.map((variant) => {
                    return (
                      <td>
                        <Button.Root styles={color} variant={variant}>
                          <Button.Label>Button</Button.Label>
                        </Button.Root>
                      </td>
                    );
                  })}
                </tr>
              );
            })}
          </tbody>
        </table>

        <hr />

        <h2>Sizes</h2>
        <table className="border-separate border-spacing-2">
          <tbody>
            <tr>
              <th></th>
              {sizes.map((size) => {
                return <th className="text-left capitalize">{size}</th>;
              })}
            </tr>

            {colors.map((color, i) => {
              return (
                <tr>
                  <th className="text-left capitalize">{i}</th>
                  {sizes.map((size) => {
                    return (
                      <td>
                        <Button.Root styles={color} size={size}>
                          <Button.Label>Button</Button.Label>
                        </Button.Root>
                      </td>
                    );
                  })}
                </tr>
              );
            })}
          </tbody>
        </table>

        <hr />

        <h2>States</h2>
        <table className="border-separate border-spacing-2">
          <tbody>
            <tr>
              <th></th>
              {variants.map((variant) => {
                return <th className="text-left capitalize">{variant}</th>;
              })}
            </tr>

            <tr>
              <th>Idle</th>
              {variants.map((variant) => {
                return (
                  <td>
                    <Button.Root variant={variant} styles={colors[0]}>
                      <Button.Label>Button</Button.Label>
                    </Button.Root>
                  </td>
                );
              })}
            </tr>

            <tr>
              <th>Loading</th>
              {variants.map((variant) => {
                return (
                  <td>
                    <Button.Root variant={variant} loading styles={colors[0]}>
                      <Button.Label>Button</Button.Label>
                    </Button.Root>
                  </td>
                );
              })}
            </tr>

            <tr>
              <th>Disabled</th>
              {variants.map((variant) => {
                return (
                  <td>
                    <Button.Root variant={variant} disabled styles={colors[0]}>
                      <Button.Label>Button</Button.Label>
                    </Button.Root>
                  </td>
                );
              })}
            </tr>
          </tbody>
        </table>

        <hr />

        <h2>Icons</h2>

        <table className="border-separate border-spacing-2">
          <tbody>
            <tr>
              <th>Idle</th>
              <td>
                <Button.Root styles={colors[0]}>
                  <Icon icon="Documentation" />
                </Button.Root>
              </td>
              <td>
                <Button.Root className="flex gap-2" styles={colors[0]}>
                  <Button.Label>Label</Button.Label>
                  <Icon icon="ArrowRight" />
                </Button.Root>
              </td>
            </tr>

            <tr>
              <th>Disabled</th>
              <td>
                <Button.Root disabled styles={colors[0]}>
                  <Icon icon="Documentation" />
                </Button.Root>
              </td>
              <td>
                <Button.Root disabled className="flex gap-2" styles={colors[0]}>
                  <Button.Label>Label</Button.Label>
                  <Icon icon="ArrowRight" />
                </Button.Root>
              </td>
            </tr>

            <tr>
              <th>Loading</th>
              <td>
                <Button.Root loading styles={colors[0]}>
                  <Icon icon="Documentation" />
                </Button.Root>
              </td>
              <td>
                <Button.Root loading className="flex gap-2" styles={colors[0]}>
                  <Button.Label>Label</Button.Label>
                  <Icon icon="ArrowRight" />
                </Button.Root>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    );
  },
};
