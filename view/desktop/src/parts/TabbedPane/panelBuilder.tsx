import * as React from "react";

import { DockviewApi } from "@repo/moss-tabs";

import { nextId } from "./defaultLayout";

export const PanelBuilder = (props: { api: DockviewApi; done: () => void }) => {
  const [parameters, setParameters] = React.useState<{
    initialWidth?: number;
    initialHeight?: number;
    maximumHeight?: number;
    maximumWidth?: number;
    minimumHeight?: number;
    minimumWidth?: number;
  }>({});
  return (
    <div>
      <div className="grid grid-cols-2">
        <div>{"Initial Width"}</div>
        <input
          className="panel-builder-input"
          type="number"
          value={parameters.initialWidth}
          onChange={(event) =>
            setParameters((_) => ({
              ..._,
              initialWidth: Number(event.target.value),
            }))
          }
        />
        <div>{"Initial Height"}</div>
        <input
          className="panel-builder-input"
          type="number"
          value={parameters.initialHeight}
          onChange={(event) =>
            setParameters((_) => ({
              ..._,
              initialHeight: Number(event.target.value),
            }))
          }
        />
        <div>{"Maximum Width"}</div>
        <input
          className="panel-builder-input"
          type="number"
          value={parameters.maximumWidth}
          onChange={(event) =>
            setParameters((_) => ({
              ..._,
              maximumWidth: Number(event.target.value),
            }))
          }
        />
        <div>{"Maximum Height"}</div>
        <input
          className="panel-builder-input"
          type="number"
          value={parameters.maximumHeight}
          onChange={(event) =>
            setParameters((_) => ({
              ..._,
              maximumHeight: Number(event.target.value),
            }))
          }
        />
        <div>{"Minimum Width"}</div>
        <input
          className="panel-builder-input"
          type="number"
          value={parameters.minimumWidth}
          onChange={(event) =>
            setParameters((_) => ({
              ..._,
              minimumWidth: Number(event.target.value),
            }))
          }
        />
        <div>{"Minimum Height"}</div>
        <input
          className="panel-builder-input"
          type="number"
          value={parameters.minimumHeight}
          onChange={(event) =>
            setParameters((_) => ({
              ..._,
              minimumHeight: Number(event.target.value),
            }))
          }
        />
      </div>
      <div>
        <button
          className="panel-builder-button"
          onClick={() => {
            props.api?.addPanel({
              id: `id_${Date.now().toString()}`,
              component: "Default",
              title: `Tab ${nextId()}`,
              renderer: "always",
              ...parameters,
            });

            props.done();
          }}
        >
          Add Panel
        </button>
        <button
          className="panel-builder-button"
          onClick={() => {
            props.done();
          }}
        >
          Cancel
        </button>
      </div>
    </div>
  );
};
