import React from "react";
import { beforeEach, describe, expect, test, vi } from "vitest";

import { act, render, waitFor } from "@testing-library/react";

import { setMockRefElement } from "../__test_utils__/utils";
import { GridviewApi } from "../../api/component.api";
import { IGridviewPanel } from "../../gridview/gridviewPanel";
import { GridviewReact, GridviewReadyEvent, IGridviewPanelProps } from "../../gridview/gridviewReact";
import { Orientation } from "../../splitview/splitview";

describe("gridview react", () => {
  let components: Record<string, React.FunctionComponent<IGridviewPanelProps>>;

  beforeEach(() => {
    components = {
      default: (props: IGridviewPanelProps) => {
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
    let api: GridviewApi | undefined;

    const onReady = (event: GridviewReadyEvent) => {
      api = event.api;
    };

    render(<GridviewReact orientation={Orientation.VERTICAL} components={components} onReady={onReady} />);

    expect(api).toBeTruthy();
  });

  test("is sized to container", () => {
    const el = document.createElement("div") as any;

    vi.spyOn(el, "clientHeight", "get").mockReturnValue(450);
    vi.spyOn(el, "clientWidth", "get").mockReturnValue(650);

    setMockRefElement(el);
    let api: GridviewApi | undefined;

    const onReady = (event: GridviewReadyEvent) => {
      api = event.api;
    };

    render(<GridviewReact orientation={Orientation.VERTICAL} components={components} onReady={onReady} />);

    expect(api!.width).toBe(650);
    expect(api!.height).toBe(450);
  });

  test("that the component can update parameters", async () => {
    let api: GridviewApi;

    const onReady = (event: GridviewReadyEvent) => {
      api = event.api;
    };

    const wrapper = render(
      <GridviewReact orientation={Orientation.VERTICAL} components={components} onReady={onReady} />
    );

    let panel: IGridviewPanel;

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

    await waitFor(() => {
      expect(wrapper.queryByText(/key=keyA,value=valueA/i)).toBeDefined();
      expect(wrapper.queryByText(/key=keyB,value=valueB/i)).toBeDefined();
    });

    act(() => {
      panel.api.updateParameters({ keyA: "valueAA", keyC: "valueC" });
    });

    await waitFor(() => {
      expect(wrapper.queryByText(/key=keyA,value=valueAA/i)).toBeDefined();
      expect(wrapper.queryByText(/key=keyB,value=valueB/i)).toBeDefined();
      expect(wrapper.queryByText(/key=keyC,value=valueC/i)).toBeDefined();
    });

    act(() => {
      panel.api.updateParameters({ keyC: null });
    });

    await waitFor(() => {
      expect(wrapper.queryByText(/key=keyA,value=valueAA/i)).toBeDefined();
      expect(wrapper.queryByText(/key=keyB,value=valueB/i)).toBeDefined();
      expect(wrapper.queryByText(/key=keyC,value=null/i)).toBeDefined();
    });

    act(() => {
      panel.api.updateParameters({ keyA: undefined });
    });

    await waitFor(() => {
      expect(wrapper.queryByText(/key=keyA/i)).toBe(null);
      expect(wrapper.queryByText(/key=keyB,value=valueB/i)).toBeDefined();
      expect(wrapper.queryByText(/key=keyC,value=null/i)).toBeDefined();
    });
  });
});
