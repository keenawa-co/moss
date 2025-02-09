import { Meta, StoryObj } from "@storybook/react";

import Input from "./Input";

const variants = ["plain", "soft", "outlined", "mixed", "bottomOutlined"] as const;
const sizes = ["sm", "md", "lg", "xl"] as const;
const meta: Meta = {
  title: "Desktop/Input",
  component: Input,
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
    // loading: false,
    // disabled: false,
    // href: undefined,
    // intent: "primary",
    // variant: "solid",
    // size: "md",
  },
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Variants: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return <Input variant={variant} placeholder="Placeholder" />;
        })}
      </div>
    );
  },
};

export const WithLabel: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-2" onSubmit={(e) => e.preventDefault()}>
        <label className="text-(--moss-primary)" htmlFor="WithLabel">
          Label
        </label>
        <Input placeholder="Placeholder" id="WithLabel" />
      </div>
    );
  },
};

export const WithCaption: Story = {
  render: () => {
    return (
      <div className="flex flex-col items-start gap-2" onSubmit={(e) => e.preventDefault()}>
        <label className="text-(--moss-primary)" htmlFor="WithCaptionWithLabel">
          Label
        </label>
        <Input placeholder="Placeholder" id="WithCaptionWithLabel" />
        <caption className="text-sm font-normal text-[rgb(113,113,122)]">Enter your name</caption>
      </div>
    );
  },
};

export const Disabled: Story = {
  render: () => {
    return (
      <div className="flex flex-col items-start gap-2" onSubmit={(e) => e.preventDefault()}>
        <label className="text-(--moss-primary)" htmlFor="DisabledWithLabel">
          Label
        </label>
        <Input placeholder="Placeholder" id="DisabledWithLabel" disabled />
        <caption className="text-sm font-normal text-[rgb(113,113,122)]">Enter your name</caption>
      </div>
    );
  },
};

export const Sizes: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-2">
        {sizes.map((size) => {
          return <Input size={size} placeholder={size} />;
        })}
      </div>
    );
  },
};

export const ValidInputs: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return <Input variant={variant} placeholder="Placeholder" data-valid />;
        })}
      </div>
    );
  },
};

export const InvalidInputs: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return <Input variant={variant} placeholder="Placeholder" data-invalid />;
        })}
      </div>
    );
  },
};
