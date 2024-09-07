import { cn } from "../utils/index";
import {
  DockviewApi,
  DockviewReact,
  DockviewReadyEvent,
  IDockviewPanelHeaderProps,
  IDockviewPanelProps,
} from "dockview";
import { useEffect, useState } from "react";
import { Table, usePanelApiMetadata, usePanelHeaderApi } from "./DockviewDebugPanel";
// default
const DefaultPanel = (props: IDockviewPanelProps) => {
  const metadata = usePanelApiMetadata(props.api);

  useEffect(() => {
    // console.log("custom panel metadata", metadata);
  }, [metadata]);
  return <div className={cn(` h-full grid place-items-center`)}>{props.api.title}</div>;
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
        "bg-gray-300": !props.api.isActive,
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
      className={cn(` h-full grid place-items-center`, {
        "bg-olive-300": props.api.isActive,
        "bg-red-300": !props.api.isActive,
      })}
    >
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
  const [dockviewApi, setDockviewApi] = useState<DockviewApi | null>(null);

  const components = {
    default: DefaultPanel,
    custom: CustomPanel,
    watermark: WatermarkPanel,
  };

  const tabs = {
    custom: CustomTab,
  };

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

  return (
    <div className="max-h-full">
      <h1>Dockview Page</h1>

      <div className="p-4 bg-green-50 flex gap-4">
        <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={addDefaultTab}>
          add default tab
        </button>
        <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2" onClick={addCustomTab}>
          add custom tab
        </button>
        <button className="border rounded bg-stone-400 hover:bg-sky-500 px-4 py-2">3</button>
      </div>

      <DockviewReact
        onReady={onReady}
        components={components}
        tabComponents={tabs}
        watermarkComponent={WatermarkPanel}
      />
    </div>
  );
};

export default DockviewPanel;
