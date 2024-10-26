import { Meta, StoryObj } from "@storybook/react";
import * as CM from "../ContextMenu/ContextMenu";
import { useState } from "react";

const meta: Meta<typeof CM.Root> = {
  title: "Shared/ContextMenu",
  component: CM.Root,
  tags: ["autodocs"],
  decorators: [
    (Story) => (
      <div className="flex justify-center">
        <Story />
      </div>
    ),
  ],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [checked1, setChecked1] = useState(true);
    const [checked2, setChecked2] = useState(false);
    const [checked3, setChecked3] = useState(true);
    const [checked4, setChecked4] = useState(false);

    return (
      <CM.Root>
        <CM.Trigger className="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
          Right click here
        </CM.Trigger>
        <CM.Portal>
          <CM.Content>
            <CM.Item label="Save Page As..." shortcut={["⇧", "⌘", "L"]}>
              Save Page As...
            </CM.Item>
            <CM.Item disabled label="Save Page As..." shortcut={["⇧", "⌘", "L"]}>
              Disabled
            </CM.Item>

            <CM.Separator />

            <CM.Item label="Move Up" icon="ArrowTop" />
            <CM.Item label="Move Down" icon="ArrowDown" />

            <CM.Separator />

            <CM.Item label="Icon hidden" icon="ArrowDown" hideIcon />

            <CM.Separator />

            <CM.CheckboxItem label="Checkbox item" checked={checked1} onCheckedChange={setChecked1} />
            <CM.CheckboxItem label="Checkbox item" checked={checked2} onCheckedChange={setChecked2} />
            <CM.CheckboxItem
              label="Checkbox item shortcut"
              shortcut={["⇧", "⌘", "L"]}
              checked={checked3}
              onCheckedChange={setChecked3}
            />
            <CM.CheckboxItem
              label="Checkbox item shortcut"
              shortcut={["⇧", "⌘", "L"]}
              checked={checked4}
              onCheckedChange={setChecked4}
            />
            <CM.CheckboxItem label="Checked disabled" checked disabled />
            <CM.CheckboxItem label="Disabled" disabled />

            <CM.Separator />

            <CM.Sub>
              <CM.SubTrigger className="ContextMenuSubTrigger" label="More Tools" />

              <CM.SubContent className="ContextMenuSubContent" sideOffset={2} alignOffset={-5}>
                <CM.Item hideIcon label="Save Page As…" />
                <CM.Item hideIcon label="Create Shortcut…" />
                <CM.Item hideIcon label="Name Window…" />

                <CM.Separator className="bg-red-400" />

                <CM.Item hideIcon label="Developer Tools" />
              </CM.SubContent>
            </CM.Sub>
          </CM.Content>
        </CM.Portal>
      </CM.Root>
    );
  },
};
