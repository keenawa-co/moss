import { Meta, StoryObj } from "@storybook/react";

import Icon from "./Icon";
import Select from "./Select";

const variants = ["outlined", "soft", "mixed", "bottomOutlined"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const countries = [
  { flag: "🇨🇩", name: "DR Congo" },
  { flag: "🇨🇬", name: "Congo Braza" },
  { flag: "🇦🇴", name: "Angola" },
  { flag: "🇫🇷", name: "France" },
  { flag: "🇬🇧", name: "United Kingdom" },
  { flag: "🇪🇸", name: "Spain" },
];
const meta: Meta = {
  title: "Desktop/Select",
  component: Select.Trigger,
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
} satisfies Meta<typeof Select.Trigger>;

export default meta;
type Story = StoryObj<typeof meta>;

type Entry = {
  flag: string;
  name: string;
};

const SelectItem = ({ entry }: { entry: Entry }) => {
  return (
    <Select.Item value={entry.name} className="items-center pl-7">
      <Select.ItemIndicator />
      <Select.ItemText>
        <span role="img" aria-label={entry.name} className="mr-2">
          {entry.flag}
        </span>
        {entry.name}
      </Select.ItemText>
    </Select.Item>
  );
};

export const Variants: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        {variants.map((variant) => (
          <tr>
            <th className="text-left">{variant}</th>
            <th>
              <Select.Root defaultValue="DR Congo">
                <Select.Trigger className="flex w-56 justify-between" variant={variant}>
                  <Select.Value placeholder="Role" />
                  <Icon icon="ChevronDown" />
                </Select.Trigger>

                <Select.Portal>
                  <Select.Content className="z-50">
                    <Select.Viewport>
                      {countries.map((country) => (
                        <SelectItem entry={country} key={country.name} />
                      ))}
                    </Select.Viewport>
                  </Select.Content>
                </Select.Portal>
              </Select.Root>
            </th>
          </tr>
        ))}
      </table>
    );
  },
};

export const Sizes: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        {sizes.map((size) => (
          <tr>
            <th>{size}</th>
            <td>
              <Select.Root defaultValue="DR Congo">
                <Select.Trigger className="flex w-56 justify-between" size={size}>
                  <Select.Value placeholder="Role" />
                  <Icon icon="ChevronDown" />
                </Select.Trigger>

                <Select.Portal>
                  <Select.Content className="z-50">
                    <Select.Viewport>
                      {countries.map((country) => (
                        <SelectItem entry={country} key={country.name} />
                      ))}
                    </Select.Viewport>
                  </Select.Content>
                </Select.Portal>
              </Select.Root>
            </td>
          </tr>
        ))}
      </table>
    );
  },
};

export const Disabled: Story = {
  render: () => {
    return (
      <Select.Root defaultValue="DR Congo">
        <Select.Trigger className="flex w-56 justify-between" disabled>
          <Select.Value placeholder="Role" />
          <Icon icon="ChevronDown" />
        </Select.Trigger>

        <Select.Portal>
          <Select.Content className="z-50">
            <Select.Viewport>
              {countries.map((country) => (
                <SelectItem entry={country} key={country.name} />
              ))}
            </Select.Viewport>
          </Select.Content>
        </Select.Portal>
      </Select.Root>
    );
  },
};

export const Valid: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        {variants.map((variant) => (
          <tr>
            <th className="text-left">{variant}</th>
            <th>
              <Select.Root defaultValue="DR Congo">
                <Select.Trigger size="md" className="flex w-56 justify-between" variant={variant} data-valid>
                  <Select.Value placeholder="Role" />
                  <Icon icon="ChevronDown" />
                </Select.Trigger>

                <Select.Portal>
                  <Select.Content className="z-50">
                    <Select.Viewport>
                      {countries.map((country) => (
                        <SelectItem entry={country} key={country.name} />
                      ))}
                    </Select.Viewport>
                  </Select.Content>
                </Select.Portal>
              </Select.Root>
            </th>
          </tr>
        ))}
      </table>
    );
  },
};

export const Invalid: Story = {
  render: () => {
    return (
      <table className="border-separate border-spacing-2">
        {variants.map((variant) => (
          <tr>
            <th className="text-left">{variant}</th>
            <th>
              <Select.Root defaultValue="DR Congo">
                <Select.Trigger size="md" className="flex w-56 justify-between" variant={variant} data-invalid>
                  <Select.Value placeholder="Role" />
                  <Icon icon="ChevronDown" />
                </Select.Trigger>

                <Select.Portal>
                  <Select.Content className="z-50">
                    <Select.Viewport>
                      {countries.map((country) => (
                        <SelectItem entry={country} key={country.name} />
                      ))}
                    </Select.Viewport>
                  </Select.Content>
                </Select.Portal>
              </Select.Root>
            </th>
          </tr>
        ))}
      </table>
    );
  },
};
