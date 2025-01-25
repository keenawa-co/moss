import { Meta, StoryObj } from "@storybook/react";

import { Button, Icon } from "./index";

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

export const All: Story = {
  render: () => {
    const variants = ["solid", "outlined", "soft", "ghost"] as const;
    const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
    const colors = [
      {
        bg: "#0073ca",
        bgHover: "#0c92eb",
        border: "#0073ca",
        text: "--moss-windowsCloseButton-background",
        ring: "#b9e0fe",
      },
      {
        bg: "#d1bf00",
        bgHover: "#ffff00",
        border: "#d1bf00",
        text: "white",
        ring: "#eeff86",
      },
      {
        bg: "#53b800",
        bgHover: "#6ee600",
        border: "#53b800",
        text: "white",
        ring: "#d0ff90",
      },
      {
        bg: "#ff0000",
        bgHover: "#ff5757",
        border: "#ff0000",
        text: "white",
        ring: "#ffc0c0",
      },
      {
        bg: "#969696",
        bgHover: "#aaaaaa",
        border: "#969696",
        text: "white",
        ring: "#e3e3e3",
      },
    ] as const;

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
