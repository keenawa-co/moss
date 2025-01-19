import { useTabbedPaneStore } from "@/store/tabbedPane";
import { DockviewApi } from "@repo/moss-tabs";

export function setGridState(api: DockviewApi) {
  const state = api.toJSON();
  const setGridState = useTabbedPaneStore.getState().setGridState;
  setGridState(state);
}
