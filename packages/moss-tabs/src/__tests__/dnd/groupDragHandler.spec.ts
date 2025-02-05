import { beforeEach, describe, expect, test, vi } from "vitest";

import { fireEvent } from "@testing-library/dom";

import { LocalSelectionTransfer, PanelTransfer } from "../../dnd/dataTransfer";
import { GroupDragHandler } from "../../dnd/groupDragHandler";
import { DockviewComponent } from "../../dockview/dockviewComponent";
import { DockviewGroupPanel } from "../../dockview/dockviewGroupPanel";

describe("groupDragHandler", () => {
  test("that the dnd transfer object is setup and torndown", () => {
    const element = document.createElement("div");

    const groupMock = vi.fn<DockviewGroupPanel, []>(() => {
      const partial: Partial<DockviewGroupPanel> = {
        id: "test_group_id",
        api: { location: { type: "grid" } } as any,
      };
      return partial as DockviewGroupPanel;
    });
    const group = new groupMock();

    const cut = new GroupDragHandler(element, { id: "test_accessor_id" } as DockviewComponent, group);

    fireEvent.dragStart(element, new Event("dragstart"));

    expect(LocalSelectionTransfer.getInstance<PanelTransfer>().hasData(PanelTransfer.prototype)).toBeTruthy();
    const transferObject = LocalSelectionTransfer.getInstance<PanelTransfer>().getData(PanelTransfer.prototype)![0];
    expect(transferObject).toBeTruthy();
    expect(transferObject.viewId).toBe("test_accessor_id");
    expect(transferObject.groupId).toBe("test_group_id");
    expect(transferObject.panelId).toBeNull();

    fireEvent.dragStart(element, new Event("dragend"));
    expect(LocalSelectionTransfer.getInstance<PanelTransfer>().hasData(PanelTransfer.prototype)).toBeFalsy();

    cut.dispose();
  });
  test("that the event is cancelled when floating and shiftKey=true", () => {
    const element = document.createElement("div");

    const groupMock = vi.fn<DockviewGroupPanel, []>(() => {
      const partial: Partial<DockviewGroupPanel> = {
        api: { location: { type: "floating" } } as any,
      };
      return partial as DockviewGroupPanel;
    });
    const group = new groupMock();

    const cut = new GroupDragHandler(element, { id: "accessor_id" } as DockviewComponent, group);

    const event = new KeyboardEvent("dragstart", { shiftKey: false });

    const spy = vi.spyOn(event, "preventDefault");
    fireEvent(element, event);
    expect(spy).toBeCalledTimes(1);

    const event2 = new KeyboardEvent("dragstart", { shiftKey: true });

    const spy2 = vi.spyOn(event2, "preventDefault");
    fireEvent(element, event);
    expect(spy2).toBeCalledTimes(0);

    cut.dispose();
  });

  test("that the event is never cancelled when the group is not floating", () => {
    const element = document.createElement("div");

    const groupMock = vi.fn<DockviewGroupPanel, []>(() => {
      const partial: Partial<DockviewGroupPanel> = {
        api: { location: { type: "grid" } } as any,
      };
      return partial as DockviewGroupPanel;
    });
    const group = new groupMock();

    const cut = new GroupDragHandler(element, { id: "accessor_id" } as DockviewComponent, group);

    const event = new KeyboardEvent("dragstart", { shiftKey: false });

    const spy = vi.spyOn(event, "preventDefault");
    fireEvent(element, event);
    expect(spy).toBeCalledTimes(0);

    const event2 = new KeyboardEvent("dragstart", { shiftKey: true });

    const spy2 = vi.spyOn(event2, "preventDefault");
    fireEvent(element, event);
    expect(spy2).toBeCalledTimes(0);

    cut.dispose();
  });
});
