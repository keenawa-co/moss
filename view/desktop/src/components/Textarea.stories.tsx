import { Meta, StoryObj } from "@storybook/react";

import Textarea from "./Textarea";

const variants = ["plain", "soft", "outlined", "mixed", "bottomOutlined"] as const;
const sizes = ["sm", "md", "lg", "xl"] as const;
const meta: Meta = {
  title: "Desktop/Textarea",
  component: Textarea,
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
    className: "h-14",
    placeholder: "Placeholder",
  },
} satisfies Meta<typeof Textarea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Variants: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return <Textarea {...args} variant={variant} />;
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
        <Textarea {...args} id="WithLabel" />
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
        <Textarea {...args} id="WithCaptionWithLabel" />
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
        <Textarea {...args} id="DisabledWithLabel" disabled />
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
          return <Textarea {...args} size={size} placeholder={size} />;
        })}
      </div>
    );
  },
};

export const Valid: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return <Textarea {...args} variant={variant} data-valid />;
        })}
      </div>
    );
  },
};

export const Invalid: Story = {
  render: (args) => {
    return (
      <div className="flex flex-col gap-2">
        {variants.map((variant) => {
          return <Textarea {...args} variant={variant} data-invalid />;
        })}
      </div>
    );
  },
};
