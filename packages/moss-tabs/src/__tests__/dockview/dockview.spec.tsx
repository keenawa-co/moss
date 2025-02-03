import React from "react";
import { beforeEach, describe, expect, test, vi } from "vitest";

import { act, render, screen, waitFor } from "@testing-library/react";

import { setMockRefElement } from "../__test_utils__/utils";
import { DockviewApi } from "../../api/component.api";
import { DockviewReact } from "../../dockview/dockview";
import { IDockviewPanel } from "../../dockview/dockviewPanel";
import { DockviewReadyEvent, IDockviewPanelProps } from "../../dockview/framework";

describe("gridview react", () => {
  let components: Record<string, React.FunctionComponent<IDockviewPanelProps>>;

  beforeEach(() => {
    components = {
      default: (props: IDockviewPanelProps) => {
        return (
          <div>
            {Object.keys(props.params).map((key) => {
              return <div key={key}>{`key=${key},value=${props.params[key]}`}</div>;
            })}
          </div>
        );
      },
    };
  });

  test("default", () => {
    let api: DockviewApi | undefined;

    const onReady = (event: DockviewReadyEvent) => {
      api = event.api;
    };

    render(<DockviewReact components={components} onReady={onReady} />);

    expect(api).toBeTruthy();
  });

  test("is sized to container", async () => {
    const el = document.createElement("div");

    vi.spyOn(el, "clientHeight", "get").mockReturnValue(450);
    vi.spyOn(el, "clientWidth", "get").mockReturnValue(650);

    const mockRef = setMockRefElement(el);

    let api: DockviewApi | undefined;
    const onReady = (event: DockviewReadyEvent) => {
      api = event.api;
    };

    render(<DockviewReact components={components} onReady={onReady} />);

    expect(mockRef).toHaveBeenCalled();
    expect(api?.width).toBe(650);
    expect(api?.height).toBe(450);
  });

  test("that the component can update parameters", async () => {
    let api: DockviewApi;

    const onReady = (event: DockviewReadyEvent) => {
      api = event.api;
    };

    const wrapper = render(<DockviewReact components={components} onReady={onReady} />);

    let panel: IDockviewPanel;

    act(() => {
      panel = api!.addPanel({
        id: "panel_1",
        component: "default",
        params: {
          keyA: "valueA",
          keyB: "valueB",
        },
      });
    });

    waitFor(() => {
      expect(wrapper.getByText(/key=keyA,value=valueA/i)).toBeDefined();
      expect(wrapper.getByText(/key=keyB,value=valueB/i)).toBeDefined();
    });

    act(() => {
      panel.api.updateParameters({ keyA: "valueAA", keyC: "valueC" });
    });

    waitFor(() => {
      expect(wrapper.getByText(/key=keyA,value=valueAA/i)).toBeDefined();
      expect(wrapper.getByText(/key=keyB,value=valueB/i)).toBeDefined();
      expect(wrapper.getByText(/key=keyC,value=valueC/i)).toBeDefined();
    });

    act(() => {
      panel.api.updateParameters({ keyC: null });
    });

    waitFor(() => {
      expect(wrapper.getByText(/key=keyA,value=valueAA/i)).toBeDefined();
      expect(wrapper.getByText(/key=keyB,value=valueB/i)).toBeDefined();
      expect(wrapper.getByText(/key=keyC,value=null/i)).toBeDefined();
    });

    act(() => {
      panel.api.updateParameters({ keyA: undefined });
    });

    waitFor(() => {
      expect(wrapper.getByText(/key=keyA/i)).not.toBeDefined();
      expect(wrapper.getByText(/key=keyB,value=valueB/i)).toBeDefined();
      expect(wrapper.getByText(/key=keyC,value=null/i)).toBeDefined();
    });
  });
});
