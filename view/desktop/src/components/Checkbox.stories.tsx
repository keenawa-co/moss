import { useState } from "react";

import { Icon } from "@repo/moss-ui";
import { Meta, StoryObj } from "@storybook/react";

import * as Checkbox from "./Checkbox";

const meta: Meta = {
  title: "Desktop/Checkbox",
  component: Checkbox.Root,
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Checkbox.Root>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithLabel: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-4">
        <div className="flex gap-2">
          <Checkbox.Root id="c1">
            <Checkbox.Indicator>
              <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c1">Checkbox 1</label>
        </div>
        <div className="flex gap-2">
          <Checkbox.Root id="c2">
            <Checkbox.Indicator>
              <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c2">Checkbox 2</label>
        </div>
        <div className="flex gap-2">
          <Checkbox.Root id="c3">
            <Checkbox.Indicator>
              <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c3">Checkbox 3</label>
        </div>
      </div>
    );
  },
};

export const Standalone: Story = {
  render: () => {
    return (
      <div className="flex gap-4">
        <Checkbox.Root id="c1">
          <Checkbox.Indicator>
            <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
          </Checkbox.Indicator>
        </Checkbox.Root>
        <Checkbox.Root id="c2">
          <Checkbox.Indicator>
            <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
          </Checkbox.Indicator>
        </Checkbox.Root>

        <Checkbox.Root id="c3">
          <Checkbox.Indicator>
            <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
          </Checkbox.Indicator>
        </Checkbox.Root>
      </div>
    );
  },
};
export const Disabled: Story = {
  render: () => {
    return (
      <div className="flex flex-col gap-4">
        <div className="flex gap-2">
          <Checkbox.Root id="c1" disabled>
            <Checkbox.Indicator>
              <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c1">Checkbox 1</label>
        </div>
        <div className="flex gap-2">
          <Checkbox.Root id="c2" disabled>
            <Checkbox.Indicator>
              <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c2">Checkbox 2</label>
        </div>
        <div className="flex gap-2">
          <Checkbox.Root id="c3" disabled>
            <Checkbox.Indicator>
              <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
            </Checkbox.Indicator>
          </Checkbox.Root>
          <label htmlFor="c3">Checkbox 3</label>
        </div>
      </div>
    );
  },
};
export const Intermediate: Story = {
  render: () => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [list, setList] = useState([
      { id: "c1", checked: false, label: "Checkbox 1" },
      { id: "c2", checked: false, label: "Checkbox 2" },
      { id: "c3", checked: false, label: "Checkbox 3" },
    ]);

    let allState: boolean | "indeterminate" = "indeterminate";

    if (list.every((item) => item.checked)) {
      allState = true;
    } else if (list.some((item) => item.checked)) {
      allState = "indeterminate";
    } else {
      allState = false;
    }

    const handleAllStateClick = () => {
      if (allState === "indeterminate") {
        setList((prev) => {
          return prev.map((item) => {
            return { ...item, checked: false };
          });
        });
      }
      if (allState === true) {
        setList((prev) => {
          return prev.map((item) => {
            return { ...item, checked: false };
          });
        });
      }
      if (allState === false) {
        setList((prev) => {
          return prev.map((item) => {
            return { ...item, checked: true };
          });
        });
      }
    };

    return (
      <div>
        <div className="mb-3 flex gap-4">
          <Checkbox.Root id="c1" checked={allState !== false} onClick={handleAllStateClick}>
            <Checkbox.Indicator>
              {allState === "indeterminate" && (
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="3"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  className="size-3.5"
                  data-state="indeterminate"
                >
                  <path d="M5 12h14"></path>
                </svg>
              )}

              {allState === true && <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />}
            </Checkbox.Indicator>
          </Checkbox.Root>
          <h2>All</h2>
        </div>
        <div className="ml-8 flex flex-col gap-4">
          {list.map((item) => {
            return (
              <div className="flex gap-2" key={item.id}>
                <Checkbox.Root
                  id={item.id}
                  checked={item.checked}
                  onClick={() => {
                    setList((prev) => {
                      return prev.map((prevItem) => {
                        if (prevItem.id === item.id) {
                          return { ...prevItem, checked: !prevItem.checked };
                        }

                        return prevItem;
                      });
                    });
                  }}
                >
                  <Checkbox.Indicator>
                    <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
                  </Checkbox.Indicator>
                </Checkbox.Root>
                <label htmlFor={item.id}>{item.label}</label>
              </div>
            );
          })}
        </div>
      </div>
    );
  },
};
