import { DockviewPanelApi } from "dockview";
import { useEffect, useState } from "react";

export function usePanelApi(api: DockviewPanelApi) {
  console.log("usePanelApi", api);
  const [state, setState] = useState({
    height: api.height,
    isActive: api.isActive,
    isFocused: api.isFocused,
    isVisible: api.isVisible,
    parameters: api.getParameters(),
    tabComponent: api.tabComponent,
    width: api.width,
    component: api.component,
    id: api.id,
  });

  useEffect(() => {
    const d1 = api.onDidActiveChange((event) => {
      setState((_) => ({
        ..._,
        isActive: event.isActive,
      }));
    });
    const d2 = api.onDidActiveGroupChange((event) => {
      setState((_) => ({
        ..._,
        isGroupActive: event.isActive,
      }));
    });
    const d3 = api.onDidDimensionsChange((event) => {
      setState((_) => ({
        ..._,
        height: event.height,
        width: event.width,
      }));
    });
    // const d4 = api.onDidFocusChange((event) => {
    //   setState((_) => ({
    //     ..._,
    //     didFocus: {
    //       count: _.didFocus.count + 1,
    //     },
    //   }));
    // });
    // const d5 = api.onDidGroupChange((event) => {
    //   setState((_) => ({
    //     ..._,
    //     groupChanged: {
    //       count: _.groupChanged.count + 1,
    //     },
    //   }));
    // });
    // const d7 = api.onDidLocationChange((event) => {
    //   setState((_) => ({
    //     ..._,
    //     location: {
    //       value: event.location,
    //       count: _.location.count + 1,
    //     },
    //   }));
    // });
    // const d8 = api.onDidRendererChange((event) => {
    //   setState((_) => ({
    //     ..._,
    //     renderer: {
    //       value: event.renderer,
    //       count: _.renderer.count + 1,
    //     },
    //   }));
    // });
    const d9 = api.onDidVisibilityChange((event) => {
      setState((_) => ({
        ..._,
        isVisible: event.isVisible,
      }));
    });

    return () => {
      d1.dispose();
      d2.dispose();
      d3.dispose();
      // d4.dispose();
      // d5.dispose();
      // d7.dispose();
      // d8.dispose();
      d9.dispose();
    };
  }, [api]);

  // close(): void;
  // setTitle(title: string): void;
  // setRenderer(renderer: DockviewPanelRenderer): void;
  // moveTo(options: {
  //     group: DockviewGroupPanel;
  //     position?: Position;
  //     index?: number;
  // }): void;
  // maximize(): void;
  // isMaximized(): boolean;
  // exitMaximized(): void;
  // /**
  //  * If you require the Window object
  //  */
  // getWindow(): Window;
  // setActive(): void;
  // setVisible(isVisible: boolean): void;
  // updateParameters(parameters: Parameters): void;

  return state;
}
