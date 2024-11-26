import { Meta, StoryObj } from "@storybook/react";
import * as DM from "../DropdownMenu/DropdownMenu";
import { useState } from "react";

const meta: Meta<typeof DM.Root> = {
  title: "Shared/DropdownMenu",
  component: DM.Root,
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
  render: () => <DefaultStory />,
};

const DefaultStory = () => {
  const [checked1, setChecked1] = useState(true);
  const [checked2, setChecked2] = useState(false);
  const [checked3, setChecked3] = useState(true);
  const [checked4, setChecked4] = useState(false);

  return (
    <DM.Root>
      <DM.Trigger className="rounded bg-sky-400 px-4 py-2 text-white">Left click here</DM.Trigger>
      <DM.Portal>
        <DM.Content>
          <DM.Item label="Save Page As..." shortcut={["⇧", "⌘", "L"]}>
            Save Page As...
          </DM.Item>
          <DM.Item disabled label="Save Page As..." shortcut={["⇧", "⌘", "L"]}>
            Disabled
          </DM.Item>

          <DM.Separator />

          <DM.Item label="Move Up" icon="ArrowTop" />
          <DM.Item label="Move Down" icon="ArrowDown" />

          <DM.Separator />

          <DM.Item label="Icon hidden" icon="ArrowDown" hideIcon />

          <DM.Separator />

          <DM.CheckboxItem label="Checkbox item" checked={checked1} onCheckedChange={setChecked1} />
          <DM.CheckboxItem label="Checkbox item" checked={checked2} onCheckedChange={setChecked2} />
          <DM.CheckboxItem
            label="Checkbox item shortcut"
            shortcut={["⇧", "⌘", "L"]}
            checked={checked3}
            onCheckedChange={setChecked3}
          />
          <DM.CheckboxItem
            label="Checkbox item shortcut"
            shortcut={["⇧", "⌘", "L"]}
            checked={checked4}
            onCheckedChange={setChecked4}
          />
          <DM.CheckboxItem label="Checked disabled" checked disabled />
          <DM.CheckboxItem label="Disabled" disabled />

          <DM.Separator />

          <DM.Sub>
            <DM.SubTrigger className="ContextMenuSubTrigger" label="More Tools" />

            <DM.SubContent className="ContextMenuSubContent" sideOffset={2} alignOffset={-5}>
              <DM.Item hideIcon label="Save Page As…" />
              <DM.Item hideIcon label="Create Shortcut…" />
              <DM.Item hideIcon label="Name Window…" />

              <DM.Separator className="bg-red-400" />

              <DM.Item hideIcon label="Developer Tools" />
            </DM.SubContent>
          </DM.Sub>
        </DM.Content>
      </DM.Portal>
    </DM.Root>
  );
};
