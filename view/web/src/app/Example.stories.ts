import type { Meta, StoryObj } from "../../../storybook/node_modules/@storybook/react";
import Example from "./Example";

const meta = {
  title: "Web/Example",
  component: Example,
  tags: ["autodocs"],
  parameters: {},
} satisfies Meta<typeof Example>;

export default meta;
type Story = StoryObj<typeof meta>;

export const ExampleComponent: Story = {};
