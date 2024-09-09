import { DockviewPanelApi } from "dockview";
import { useEffect, useState } from "react";

export function usePanelApi(api: DockviewPanelApi) {
  const [state, setState] = useState({
    id: api.id,
    isActive: api.isActive,
    isGroupActive: api.isGroupActive,
    isVisible: api.isVisible,
    isFocused: api.isFocused,
    parameters: api.getParameters(),
    height: api.height,
    width: api.width,
    tabComponent: api.tabComponent,
    location: api.location,
    renderer: api.renderer,
    component: api.component,
    title: api.title,
  });

  const updateState = (key: string, value: any) => {
    setState((prevState) => ({
      ...prevState,
      [key]: value,
    }));
  };

  useEffect(() => {
    const disposables = [
      api.onDidActiveChange((event) => updateState("isActive", event.isActive)),
      api.onDidActiveGroupChange((event) => updateState("isGroupActive", event.isActive)),
      api.onDidLocationChange((event) => updateState("location", event.location)),
      api.onDidRendererChange((event) => updateState("renderer", event.renderer)),
      api.onDidVisibilityChange((event) => updateState("isVisible", event.isVisible)),
      api.onDidParametersChange((event) => updateState("parameters", event)),
      api.onDidFocusChange((event) => updateState("isFocused", event.isFocused)),
      api.onDidTitleChange((event) => updateState("title", event.title)),
      api.onDidDimensionsChange((event) => {
        setState((prevState) => ({
          ...prevState,
          height: event.height,
          width: event.width,
        }));
      }),
    ];

    return () => {
      disposables.forEach((d) => d.dispose());
    };
  }, [api]);

  return state;
}
