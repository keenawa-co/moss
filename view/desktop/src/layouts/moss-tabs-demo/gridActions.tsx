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
    <div className="absolute left-0 top-0 z-[9999] h-full w-full">
      <div
        ref={ref}
        className="bg-black absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform p-2.5 text-white"
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

  const onAddPanel = (options?: { advanced: boolean }) => {
    if (options?.advanced) {
      popover.open(({ close }) => {
        return <PanelBuilder api={props.api!} done={close} />;
      });
    } else {
      props.api?.addPanel({
        id: `id_${Date.now().toString()}`,
        component: "default",
        title: `Tab ${nextId()}`,
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
      <div className="button-group">
        <button className="text-button" onClick={() => onAddPanel()}>
          Add Panel
        </button>
        <button className="demo-icon-button" onClick={() => onAddPanel({ advanced: true })}>
          <span className="material-symbols-outlined">tune</span>
        </button>
      </div>
      <button className="text-button" onClick={onAddGroup}>
        Add Group
      </button>
      <span className="button-action">
        <button
          className={props.hasCustomWatermark ? "demo-button selected" : "demo-button"}
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
      <span className="flex-grow" />
      <div className="flex">
        <span className="pr-1">Group Gap</span>
        <input
          style={{ width: 40 }}
          type="number"
          min={0}
          max={99}
          step={1}
          value={gap}
          onChange={(event) => setGap(Number(event.target.value))}
        />
      </div>
    </div>
  );
};
