import { useState } from "react";

import { Meta, StoryObj } from "@storybook/react";

import Input from "./Input";

const variants = ["plain", "soft", "outlined", "mixed", "bottomOutlined"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
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
    disabled: false,
    variant: "outlined",
    size: "md",
    placeholder: "Placeholder",
  },
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Variants: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return <Input {...args} variant={variant} />;
        })}
      </div>
    );
  },
};

export const WithLabel: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col gap-2" onSubmit={(e) => e.preventDefault()}>
        <label className="text-(--moss-primary)" htmlFor="WithLabel">
          Label
        </label>
        <Input {...args} id="WithLabel" />
      </div>
    );
  },
};

export const WithCaption: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col items-start gap-2" onSubmit={(e) => e.preventDefault()}>
        <label className="text-(--moss-primary)" htmlFor="WithCaptionWithLabel">
          Label
        </label>
        <Input {...args} id="WithCaptionWithLabel" />
        <caption className="text-sm font-normal text-[rgb(113,113,122)]">Enter your name</caption>
      </div>
    );
  },
};

export const Disabled: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col items-start gap-2" onSubmit={(e) => e.preventDefault()}>
        <label className="text-(--moss-primary)" htmlFor="DisabledWithLabel">
          Label
        </label>
        <Input {...args} id="DisabledWithLabel" disabled />
        <caption className="text-sm font-normal text-[rgb(113,113,122)]">Enter your name</caption>
      </div>
    );
  },
};

export const Sizes: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col gap-2">
        {sizes.map((size) => {
          return <Input {...args} size={size} placeholder={size} />;
        })}
      </div>
    );
  },
};

export const ValidInputs: Story = {
  render: (args) => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [value, setValue] = useState("Valid input");
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return (
            <Input {...args} variant={variant} data-valid onChange={(e) => setValue(e.target.value)} value={value} />
          );
        })}
      </div>
    );
  },
};

export const InvalidInputs: Story = {
  render: (args) => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [value, setValue] = useState("Invalid input");
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return (
            <Input {...args} variant={variant} data-invalid onChange={(e) => setValue(e.target.value)} value={value} />
          );
        })}
      </div>
    );
  },
};
