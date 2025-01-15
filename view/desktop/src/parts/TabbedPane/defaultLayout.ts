import { DockviewApi } from "@repo/moss-tabs";

export const nextId = (() => {
  let counter = 0;

  return () => counter++;
})();

export function defaultConfig(api: DockviewApi) {
  const panel1 = api.addPanel({
    id: "Home",
    component: "Home",
    renderer: "always",
    title: "Home",
  });

  api.addPanel({
    id: "Settings",
    component: "Settings",
    title: "Settings",
    position: { referencePanel: panel1 },
  });

  api.addPanel({
    id: "Logs",
    component: "Logs",
    title: "Logs",
    position: { referencePanel: panel1 },
  });

  const panel4 = api.addPanel({
    id: "panel_4",
    component: "Default",
    title: "Panel 4",
    position: { referencePanel: panel1, direction: "right" },
  });

  const panel5 = api.addPanel({
    id: "panel_5",
    component: "Default",
    title: "Panel 5",
    position: { referencePanel: panel4 },
  });

  const panel6 = api.addPanel({
    id: "panel_6",
    component: "Default",
    title: "Panel 6",
    position: { referencePanel: panel5, direction: "below" },
  });

  const panel7 = api.addPanel({
    id: "panel_7",
    component: "Default",
    title: "Panel 7",
    position: { referencePanel: panel6, direction: "left" },
  });

  api.addPanel({
    id: "panel8",
    component: "Default",
    title: "Panel 8",
    position: { referencePanel: panel7, direction: "below" },
  });

  panel1.api.setActive();
}
