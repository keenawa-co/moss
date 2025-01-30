import { describe, expect, test, vi } from "vitest";

import { DockviewGroupPanelApi } from "../../api/dockviewGroupPanelApi";
import { DockviewGroupPanel } from "../../dockview/dockviewGroupPanel";
import { DockviewGroupPanelModel } from "../../dockview/dockviewGroupPanelModel";
import { ReactHeaderActionsRendererPart } from "../../dockview/headerActionsRenderer";

describe("headerActionsRenderer", () => {
  test("#1", () => {
    const groupviewMock = vi.fn<Partial<DockviewGroupPanelModel>, []>(() => {
      return {
        onDidAddPanel: vi.fn(),
        onDidRemovePanel: vi.fn(),
        onDidActivePanelChange: vi.fn(),
      };
    });

    const groupview = new groupviewMock() as DockviewGroupPanelModel;

    const groupPanelMock = vi.fn<Partial<DockviewGroupPanel>, []>(() => {
      return {
        api: {} as DockviewGroupPanelApi as any,
        model: groupview,
      };
    });

    const groupPanel = new groupPanelMock() as DockviewGroupPanel;

    const cut = new ReactHeaderActionsRendererPart(
      vi.fn(),
      {
        addPortal: vi.fn(),
      },
      groupPanel
    );

    expect(cut.element.childNodes.length).toBe(0);
    expect(cut.element.className).toBe("dv-react-part");
    expect(cut.part).toBeUndefined();

    cut.init({
      containerApi: <any>vi.fn(),
      api: <any>{
        onDidActiveChange: vi.fn(),
      },
    });

    const update = vi.fn();

    vi.spyOn(cut.part!, "update").mockImplementation(update);

    cut.update({ params: { valueA: "A" } });

    expect(update).toBeCalledWith({ valueA: "A" });
  });
});
