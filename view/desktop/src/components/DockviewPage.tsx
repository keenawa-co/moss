import { cn } from "../utils/index";
import {
  DockviewApi,
  DockviewReact,
  DockviewReadyEvent,
  IDockviewPanelHeaderProps,
  IDockviewPanelProps,
  SerializedDockview,
} from "dockview";
import { useEffect, useState } from "react";
import { Table, usePanelApiMetadata, usePanelHeaderApi } from "./DockviewDebugPanel";
import * as TestComps from "./TestPages/index";

// default
const DefaultPanel = (props: IDockviewPanelProps) => {
  // const metadata = usePanelApiMetadata(props.api);

  // useEffect(() => {
  //   // console.log("custom panel metadata", metadata);
  // }, [metadata]);
  return (
    <div className={cn(`h-full grid place-items-center text-3xl`)}>
      <div>
        {props.api.title} | <span className="font-black text-sky-400">{new Date().getSeconds().toString()}</span>
      </div>
    </div>
  );
};

// custom
const CustomTab = (props: IDockviewPanelHeaderProps) => {
  console.log("custom tab props", props.api);

  const metadata = usePanelHeaderApi(props.api);

  useEffect(() => {
    console.log("custom tab  metadata", metadata);
  }, [metadata]);

  return (
    <div
      className={cn(`flex items-center justify-between px-4 h-full`, {
        "bg-olive-300": props.api.isActive,
        "bg-stone-400": !props.api.isActive,
      })}
    >
      <div>custom tab</div>

      <div className="ml-4 px-1 hover:bg-red-500 rounded" onClick={() => props.api.close()}>
        X
      </div>
    </div>
  );
};

const CustomPanel = (props: IDockviewPanelProps) => {
  const metadata = usePanelApiMetadata(props.api);

  useEffect(() => {
    // console.log("custom panel metadata", metadata);
  }, [metadata]);

  return (
    <div
      className={cn(`h-full flex flex-col justify-center items-center`, {
        "bg-olive-300": props.api.isActive,
        "bg-red-300": !props.api.isActive,
      })}
    >
      <div className="text-3xl font-bold mb-12">{props.api.isActive ? "Active now" : "Inactive"}</div>
      <Table data={metadata} />
    </div>
  );
};

// watermark
const WatermarkPanel = () => {
  return (
    <div className="h-full w-full grid place-items-center bg-red-400">
      <div>
        <div className="text-center text-5xl font-bold">No content chosen</div>
        <img className="w-80 mx-auto" src="https://media.tenor.com/OA8KFcZxPjsAAAAi/sad-emoji.gif" alt="" />
      </div>
    </div>
  );
};

const DockviewPanel = () => {
  console.log(TestComps);
  const [dockviewApi, setDockviewApi] = useState<DockviewApi | null>(null);

  const onReady = (event: DockviewReadyEvent) => {
    setDockviewApi(event.api);

    event.api.addPanel({
      id: `defaultPanel1`,
      title: "Default Panel 1",
      component: "default",
    });

    event.api.addPanel({
      id: `defaultPanel2`,
      title: "Default Panel 2",
      component: "default",
    });

    event.api.addPanel({
      id: `defaultPanel3`,
      title: "Default Panel 3",
      component: "default",
    });
  };

  // registration
  const panels = {
    default: DefaultPanel,
    custom: CustomPanel,
    watermark: WatermarkPanel,
    ...TestComps,
  };

  const tabs = {
    custom: CustomTab,
  };

  // actions
  const addDefaultTab = () => {
    dockviewApi?.addPanel({
      id: `id_${Date.now().toString()}`,
      title: "Default",
      component: "default",
    });
  };

  const addCustomTab = () => {
    dockviewApi?.addPanel({
      id: `id_${Date.now().toString()}`,
      title: "Custom",
      component: "custom",
      tabComponent: "custom",
    });
  };

  const saveLayout = () => {
    if (!dockviewApi) return;

    const layout: SerializedDockview = dockviewApi.toJSON();
    localStorage.setItem("dockviewLayout", JSON.stringify(layout));
  };

  const loadLayout = () => {
    if (!dockviewApi) return;

    const layout = localStorage.getItem("dockviewLayout");
    if (!layout) {
      console.log("No layout to load");
      return;
    }

    dockviewApi.clear();

    const parsedLayout = JSON.parse(layout) as SerializedDockview;
    dockviewApi.fromJSON(parsedLayout);
  };

  const addTestDynamicPanel1 = () => {
    dockviewApi?.addPanel({
      id: `id_${Date.now().toString()}`,
      title: "Test Dynamic 1",
      component: "TestPage1",
    });
  };
  const addTestDynamicPanel2 = () => {
    dockviewApi?.addPanel({
      id: `id_${Date.now().toString()}`,
      title: "Test Dynamic 2",
      component: "TestPage2",
    });
  };

  return (
    <div className="max-h-full">
      <h1>Dockview Page</h1>

      <div className="p-4 flex justify-between">
        <fieldset className="flex gap-4">
          <legend>Actions</legend>
          <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={addDefaultTab}>
            add default tab
          </button>

          <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={addCustomTab}>
            add custom tab
          </button>
        </fieldset>

        <fieldset>
          <legend>Test Dynamic Panel</legend>
          <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={addTestDynamicPanel1}>
            add test dynamic panel 1
          </button>
          <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={addTestDynamicPanel2}>
            add test dynamic panel 2
          </button>
        </fieldset>

        <fieldset className="flex gap-4">
          <legend>Layout</legend>
          <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={saveLayout}>
            Save
          </button>
          <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={loadLayout}>
            Load
          </button>
        </fieldset>
      </div>

      <DockviewReact onReady={onReady} components={panels} tabComponents={tabs} watermarkComponent={WatermarkPanel} />
    </div>
  );
};

export default DockviewPanel;
