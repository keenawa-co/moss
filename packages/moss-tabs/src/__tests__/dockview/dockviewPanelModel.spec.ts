import { beforeEach, describe, expect, Mock, test, vi } from "vitest";

import { fromPartial } from "@total-typescript/shoehorn";

import { DefaultTab } from "../../dockview/components/tab/defaultTab";
import { DockviewComponent } from "../../dockview/dockviewComponent";
import { DockviewPanelModel } from "../../dockview/dockviewPanelModel";
import { IContentRenderer, ITabRenderer } from "../../dockview/types";

describe("dockviewGroupPanel", () => {
  let contentMock: Mock<IContentRenderer>; //Type 'IContentRenderer' does not satisfy the constraint 'Procedure'. Type 'IContentRenderer' provides no match for the signature '(...args: any[]): any'.ts(2344)
  let tabMock: Mock<ITabRenderer>; //Type 'ITabRenderer' does not satisfy the constraint 'Procedure'. Type 'ITabRenderer' provides no match for the signature '(...args: any[]): any'.ts(2344)
  let accessorMock: DockviewComponent;

  beforeEach(() => {
    contentMock = vi.fn<IContentRenderer>(() => {
      // Type 'IContentRenderer' does not satisfy the constraint 'Procedure'. Type 'IContentRenderer' provides no match for the signature '(...args: any[]): any'.ts(2344)
      const partial: Partial<IContentRenderer> = {
        element: document.createElement("div"),
        dispose: vi.fn(),
        update: vi.fn(),
      };
      return partial as IContentRenderer;
    });

    tabMock = vi.fn<ITabRenderer>(() => {
      //Type 'ITabRenderer' does not satisfy the constraint 'Procedure'. Type 'ITabRenderer' provides no match for the signature '(...args: any[]): any'.ts(2344)

      const partial: Partial<ITabRenderer> = {
        element: document.createElement("div"),
        dispose: vi.fn(),
        update: vi.fn(),
        init: vi.fn(),
      };
      return partial as ITabRenderer;
    });

    accessorMock = fromPartial<DockviewComponent>({
      options: {
        createComponent(options: { id: string; name: string }): IContentRenderer {
          switch (options.name) {
            case "contentComponent":
              return new contentMock(options.id, options.name); //Argument of type '[string, "contentComponent"]' is not assignable to parameter of type 'never'.ts(2345)
            default:
              throw new Error(`unsupported`);
          }
        },
        createTabComponent(options: { id: string; name: string }): ITabRenderer {
          switch (options.name) {
            case "tabComponent":
              return new tabMock(options.id, options.name); //Argument of type '[string, "tabComponent"]' is not assignable to parameter of type 'never'.ts(2345)
            default:
              throw new Error(`unsupported`);
          }
        },
      },
    });
  });

  test("that dispose is called on content and tab renderers when present", () => {
    const cut = new DockviewPanelModel(accessorMock, "id", "contentComponent", "tabComponent");

    cut.dispose();

    expect(cut.content.dispose).toHaveBeenCalled();
    expect(cut.tab.dispose).toHaveBeenCalled();
  });

  test("that update is called on content and tab renderers when present", () => {
    const cut = new DockviewPanelModel(accessorMock, "id", "contentComponent", "tabComponent");

    cut.update({
      params: {},
    });

    expect(cut.content.update).toHaveBeenCalled();
    expect(cut.tab.update).toHaveBeenCalled();
  });

  test("that the default tab is created", () => {
    accessorMock = fromPartial<DockviewComponent>({
      options: {
        createComponent(options: { id: string; name: string }): IContentRenderer {
          switch (options.name) {
            case "contentComponent":
              return new contentMock(options.id, options.name);
            default:
              throw new Error(`unsupported`);
          }
        },
        createTabComponent(options: { id: string; name: string }): ITabRenderer {
          switch (options.name) {
            case "tabComponent":
              return new tabMock(options.id, options.name);
            default:
              throw new Error(`unsupported`);
          }
        },
      },
    });

    const cut = new DockviewPanelModel(accessorMock, "id", "contentComponent", "tabComponent");

    expect(cut.tab).toMatchObject({
      dispose: expect.any(Function),
      element: expect.anything(),
      init: expect.any(Function),
      update: expect.any(Function),
    });
  });

  test("that the provided default tab is chosen when no implementation is provided", () => {
    accessorMock = fromPartial<DockviewComponent>({
      options: {
        defaultTabComponent: "tabComponent",
        createComponent(options: { id: string; name: string }): IContentRenderer {
          switch (options.name) {
            case "contentComponent":
              return new contentMock(options.id, options.name);
            default:
              throw new Error(`unsupported`);
          }
        },
        createTabComponent(options: { id: string; name: string }): ITabRenderer {
          switch (options.name) {
            case "tabComponent":
              return new tabMock(options.id, options.name);
            default:
              throw new Error(`unsupported`);
          }
        },
      },
    });

    const cut = new DockviewPanelModel(accessorMock, "id", "contentComponent");

    expect(cut.tab).toMatchObject({
      dispose: expect.any(Function),
      element: expect.anything(),
      init: expect.any(Function),
      update: expect.any(Function),
    });
  });

  test("that is library default tab instance is created when no alternative exists", () => {
    accessorMock = fromPartial<DockviewComponent>({
      options: {
        createComponent(options: { id: string; name: string }): IContentRenderer {
          switch (options.name) {
            case "contentComponent":
              return new contentMock(options.id, options.name);
            default:
              throw new Error(`unsupported`);
          }
        },
      },
    });

    const cut = new DockviewPanelModel(accessorMock, "id", "contentComponent");

    expect(cut.tab instanceof DefaultTab).toBeTruthy();
  });

  test("that the default content is created", () => {
    accessorMock = fromPartial<DockviewComponent>({
      options: {
        createComponent(options: { id: string; name: string }): IContentRenderer {
          switch (options.name) {
            case "contentComponent":
              return new contentMock(options.id, options.name);
            default:
              throw new Error(`unsupported`);
          }
        },
        createTabComponent(options: { id: string; name: string }): ITabRenderer {
          switch (options.name) {
            case "tabComponent":
              return new tabMock(options.id, options.name);
            default:
              throw new Error(`unsupported`);
          }
        },
      },
    });

    const cut = new DockviewPanelModel(accessorMock, "id", "contentComponent");

    expect(cut.content).toMatchObject({
      dispose: expect.any(Function),
      element: expect.anything(),
      update: expect.any(Function),
    });
  });
});
