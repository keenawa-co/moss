import * as React from "react";
import { createRoot, Root } from "react-dom/client";

import { DockviewApi } from "@repo/moss-tabs";

import { defaultConfig, nextId } from "./defaultLayout";
import { PanelBuilder } from "./panelBuilder";

let mount = document.querySelector(".popover-anchor") as HTMLElement | null;

if (!mount) {
  mount = document.createElement("div");
  mount.className = "popover-anchor";
  document.body.insertBefore(mount, document.body.firstChild);
}

const PopoverComponent = (props: { close: () => void; component: React.FC<{ close: () => void }> }) => {
  const ref = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    const handler = (ev: MouseEvent) => {
      let target = ev.target as HTMLElement;

      while (target.parentElement) {
        if (target === ref.current) {
          return;
        }
        target = target.parentElement;
      }

      props.close();
    };

    window.addEventListener("mousedown", handler);

    return () => {
      window.removeEventListener("mousedown", handler);
    };
  }, []);

  return (
    <div className="absolute top-0 left-0 z-[9999] h-full w-full">
      <div
        ref={ref}
        className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 transform bg-black p-2.5 text-white"
      >
        <props.component close={props.close} />
      </div>
    </div>
  );
};

function usePopover() {
  return {
    open: (Component: React.FC<{ close: () => void }>) => {
      const el = document.createElement("div");
      mount!.appendChild(el);
      const root = createRoot(el);

      root.render(
        <PopoverComponent
          component={Component}
          close={() => {
            root.unmount();
            el.remove();
          }}
        />
      );
    },
  };
}

export const GridActions = (props: {
  api?: DockviewApi;
  hasCustomWatermark: boolean;
  toggleCustomWatermark: () => void;
}) => {
  const onClear = () => {
    props.api?.clear();
  };

  const onLoad = () => {
    const state = localStorage.getItem("dv-demo-state");
    if (state) {
      try {
        props.api?.fromJSON(JSON.parse(state));
      } catch (err) {
        console.error("failed to load state", err);
        localStorage.removeItem("dv-demo-state");
      }
    }
  };

  const onSave = () => {
    if (props.api) {
      const state = props.api.toJSON();
      console.log(state);

      localStorage.setItem("dv-demo-state", JSON.stringify(state));
    }
  };

  const onReset = () => {
    if (props.api) {
      try {
        props.api.clear();
        defaultConfig(props.api);
      } catch (err) {
        localStorage.removeItem("dv-demo-state");
      }
    }
  };

  const popover = usePopover();

  const onAddPanel = (options?: { advanced?: boolean; type?: string }) => {
    const panelType = options?.type;
    if (panelType && props.api?.getPanel(panelType) !== undefined) {
      return;
    }

    if (options?.advanced) {
      popover.open(({ close }) => {
        return <PanelBuilder api={props.api!} done={close} />;
      });
    } else {
      props.api?.addPanel({
        id: panelType && panelType !== "nested" ? panelType : `id_${Date.now().toString()}`,
        component: options?.type ?? "Default",
        title: options?.type ?? `Tab ${nextId()}`,
        renderer: "always",
      });
    }
  };

  const onAddGroup = () => {
    props.api?.addGroup();
  };

  const [gap, setGap] = React.useState(0);

  React.useEffect(() => {
    props.api?.setGap(gap);
  }, [gap, props.api]);

  return (
    <div className="action-container">
      <button className="text-button" onClick={() => onAddPanel({ type: "Home" })}>
        Home
      </button>
      <button className="text-button" onClick={() => onAddPanel({ type: "Settings" })}>
        Settings
      </button>
      <button className="text-button" onClick={() => onAddPanel({ type: "Logs" })}>
        Logs
      </button>
      <span className="grow" />
      <div className="button-group">
        <button className="text-button" onClick={() => onAddPanel()}>
          Add Panel
        </button>
        <button className="demo-icon-button !rounded" onClick={() => onAddPanel({ advanced: true })}>
          <span className="material-symbols-outlined">tune</span>
        </button>
      </div>
      <button className="text-button" onClick={() => onAddPanel({ type: "nested" })}>
        Add Nested Panel
      </button>
      <button className="text-button" onClick={onAddGroup}>
        Add Group
      </button>
      <span className="button-action">
        <button
          className={props.hasCustomWatermark ? "demo-button selected !rounded" : "demo-button !rounded"}
          onClick={props.toggleCustomWatermark}
        >
          Use Custom Watermark
        </button>
      </span>
      <button className="text-button" onClick={onClear}>
        Clear
      </button>
      <button className="text-button" onClick={onLoad}>
        Load
      </button>
      <button className="text-button" onClick={onSave}>
        Save
      </button>
      <button className="text-button" onClick={onReset}>
        Reset
      </button>
      <span className="grow" />
      <div className="flex items-center">
        <span className="pr-1 text-[var(--moss-activegroup-visiblepanel-tab-color)]">Grid Gap</span>
        <input
          className="w-10 text-center"
          type="number"
          min={0}
          max={99}
          step={1}
          value={gap}
          onChange={(event) => setGap(Number(event.target.value))}
        />
        <button className="text-button" onClick={() => setGap(0)}>
          Reset
        </button>
      </div>
    </div>
  );
};
