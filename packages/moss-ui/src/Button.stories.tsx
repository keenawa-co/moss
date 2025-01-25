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
    const intents = ["primary", "warning", "success", "danger", "neutral"] as const;

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

            {intents.map((intent) => {
              return (
                <tr>
                  <th className="text-left capitalize">{intent}</th>
                  {variants.map((variant) => {
                    return (
                      <td>
                        <Button.Root intent={intent} variant={variant}>
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

            {intents.map((intent) => {
              return (
                <tr>
                  <th className="text-left capitalize">{intent}</th>
                  {sizes.map((size) => {
                    return (
                      <td>
                        <Button.Root intent={intent} size={size}>
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
                    <Button.Root variant={variant}>
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
                    <Button.Root variant={variant} loading>
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
                    <Button.Root variant={variant} disabled>
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
                <Button.Root>
                  <Icon icon="Documentation" />
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
              <th>Disabled</th>
              <td>
                <Button.Root disabled>
                  <Icon icon="Documentation" />
                </Button.Root>
              </td>
              <td>
                <Button.Root disabled className="flex gap-2">
                  <Button.Label>Label</Button.Label>
                  <Icon icon="ArrowRight" />
                </Button.Root>
              </td>
            </tr>

            <tr>
              <th>Loading</th>
              <td>
                <Button.Root loading>
                  <Icon icon="Documentation" />
                </Button.Root>
              </td>
              <td>
                <Button.Root loading className="flex gap-2">
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
