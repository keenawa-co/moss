import { beforeEach, describe, expect, test, vi } from "vitest";

import { fireEvent } from "@testing-library/dom";
import { fromPartial } from "@total-typescript/shoehorn";

import { DockviewApi } from "../../../../api/component.api";
import { DockviewPanelApi, TitleEvent } from "../../../../api/dockviewPanelApi";
import { DefaultTab } from "../../../../dockview/components/tab/defaultTab";
import { Emitter } from "../../../../events";

describe("defaultTab", () => {
  test("that title updates", () => {
    const cut = new DefaultTab();

    let el = cut.element.querySelector(".dv-default-tab-content");
    expect(el).toBeTruthy();
    expect(el!.textContent).toBe("");

    const onDidTitleChange = new Emitter<TitleEvent>();

    const api = fromPartial<DockviewPanelApi>({
      onDidTitleChange: onDidTitleChange.event,
    });
    const containerApi = fromPartial<DockviewApi>({});

    cut.init({
      api,
      containerApi,
      params: {},
      title: "title_abc",
    });

    el = cut.element.querySelector(".dv-default-tab-content");
    expect(el).toBeTruthy();
    expect(el!.textContent).toBe("title_abc");

    onDidTitleChange.fire({ title: "title_def" });

    expect(el!.textContent).toBe("title_def");
  });

  test("that click closes tab", () => {
    const cut = new DefaultTab();

    const api = fromPartial<DockviewPanelApi>({
      onDidTitleChange: vi.fn(),
      close: vi.fn(),
    });
    const containerApi = fromPartial<DockviewApi>({});

    cut.init({
      api,
      containerApi,
      params: {},
      title: "title_abc",
    });

    let el = cut.element.querySelector(".dv-default-tab-action");

    fireEvent.pointerDown(el!);
    expect(api.close).toHaveBeenCalledTimes(0);

    fireEvent.click(el!);
    expect(api.close).toHaveBeenCalledTimes(1);
  });
});
