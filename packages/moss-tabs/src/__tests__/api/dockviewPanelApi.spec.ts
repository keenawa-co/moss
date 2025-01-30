import { beforeEach, describe, expect, test, vi } from "vitest";

import { fromPartial } from "@total-typescript/shoehorn";

import { DockviewPanelApiImpl } from "../../api/dockviewPanelApi";
import { DockviewComponent } from "../../dockview/dockviewComponent";
import { DockviewGroupPanel } from "../../dockview/dockviewGroupPanel";
import { DockviewPanel } from "../../dockview/dockviewPanel";

describe("groupPanelApi", () => {
  test("title", () => {
    const accessor = fromPartial<DockviewComponent>({
      onDidAddPanel: vi.fn(),
      onDidRemovePanel: vi.fn(),
      options: {},
    });

    const panelMock = vi.fn<DockviewPanel>(() => {
      return {
        update: vi.fn(),
        setTitle: vi.fn(),
      } as any;
    });

    const panel = new panelMock();
    const group = fromPartial<DockviewGroupPanel>({
      api: {
        onDidVisibilityChange: vi.fn(),
        onDidLocationChange: vi.fn(),
        onDidActiveChange: vi.fn(),
      },
    });

    const cut = new DockviewPanelApiImpl(panel, group, <DockviewComponent>accessor, "fake-component");

    cut.setTitle("test_title");
    expect(panel.setTitle).toBeCalledTimes(1);
    expect(panel.setTitle).toBeCalledWith("test_title");
  });

  test("updateParameters", () => {
    const groupPanel: Partial<DockviewPanel> = {
      id: "test_id",
      update: vi.fn(),
    };

    const accessor = fromPartial<DockviewComponent>({
      onDidAddPanel: vi.fn(),
      onDidRemovePanel: vi.fn(),
      options: {},
    });

    const groupViewPanel = new DockviewGroupPanel(<DockviewComponent>accessor, "", {});

    const cut = new DockviewPanelApiImpl(
      <DockviewPanel>groupPanel,
      <DockviewGroupPanel>groupViewPanel,
      <DockviewComponent>accessor,
      "fake-component"
    );

    cut.updateParameters({ keyA: "valueA" });

    expect(groupPanel.update).toHaveBeenCalledWith({
      params: { keyA: "valueA" },
    });
    expect(groupPanel.update).toHaveBeenCalledTimes(1);
  });

  test("onDidGroupChange", () => {
    const groupPanel: Partial<DockviewPanel> = {
      id: "test_id",
    };

    const accessor = fromPartial<DockviewComponent>({
      onDidAddPanel: vi.fn(),
      onDidRemovePanel: vi.fn(),
      options: {},
    });

    const groupViewPanel = new DockviewGroupPanel(<DockviewComponent>accessor, "", {});

    const cut = new DockviewPanelApiImpl(
      <DockviewPanel>groupPanel,
      <DockviewGroupPanel>groupViewPanel,
      <DockviewComponent>accessor,
      "fake-component"
    );

    let events = 0;

    const disposable = cut.onDidGroupChange(() => {
      events++;
    });

    expect(events).toBe(0);
    expect(cut.group).toBe(groupViewPanel);

    const groupViewPanel2 = new DockviewGroupPanel(<DockviewComponent>accessor, "", {});
    cut.group = groupViewPanel2;
    expect(events).toBe(1);
    expect(cut.group).toBe(groupViewPanel2);

    disposable.dispose();
  });
});
