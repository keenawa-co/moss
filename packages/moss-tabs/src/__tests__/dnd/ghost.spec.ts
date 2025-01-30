import { beforeEach, describe, expect, test, vi } from "vitest";

import { addGhostImage } from "../../dnd/ghost";

describe("ghost", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.clearAllTimers();
  });

  test("that a custom class is added, the element is added to the document and all is removed afterwards", async () => {
    const dataTransferMock = vi.fn<Partial<DataTransfer>, []>(() => {
      return {
        setDragImage: vi.fn(),
      };
    });

    const element = document.createElement("div");
    const dataTransfer = <DataTransfer>new dataTransferMock();

    addGhostImage(dataTransfer, element);

    expect(element.className).toBe("dv-dragged");
    expect(element.parentElement).toBe(document.body);
    expect(dataTransfer.setDragImage).toBeCalledTimes(1);
    expect(dataTransfer.setDragImage).toBeCalledWith(element, 0, 0);

    await vi.runAllTimersAsync();

    expect(element.className).toBe("");
    expect(element.parentElement).toBe(null);
  });
});
