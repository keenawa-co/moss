import React from "react";

import { DockviewApi, IDockviewPanel } from "@repo/moss-tabs";

const PanelAction = (props: { panels: string[]; api: DockviewApi; activePanel?: string; panelId: string }) => {
  const onClick = () => {
    props.api.getPanel(props.panelId)?.focus();
  };

  React.useEffect(() => {
    const panel = props.api.getPanel(props.panelId);
    if (panel) {
      const disposable = panel.api.onDidVisibilityChange((event) => {
        setVisible(event.isVisible);
      });
      setVisible(panel.api.isVisible);

      return () => {
        disposable.dispose();
      };
    }
  }, [props.api, props.panelId]);

  const [panel, setPanel] = React.useState<IDockviewPanel | undefined>(undefined);

  React.useEffect(() => {
    const list = [
      props.api.onDidLayoutFromJSON(() => {
        setPanel(props.api.getPanel(props.panelId));
      }),
    ];

    if (panel) {
      const disposable = panel.api.onDidVisibilityChange((event) => {
        setVisible(event.isVisible);
      });
      setVisible(panel.api.isVisible);

      list.push(disposable);
    }

    setPanel(props.api.getPanel(props.panelId));

    return () => {
      list.forEach((l) => l.dispose());
    };
  }, [props.api, props.panelId]);

  const [visible, setVisible] = React.useState<boolean>(true);

  const [isPopupOpen, setIsPopupOpen] = useState(false);

  const togglePopup = () => {
    setIsPopupOpen(!isPopupOpen);
  };

  return (
    <div className="button-action">
      <div className="flex">
        <button
          className={props.activePanel === props.panelId ? "demo-button selected" : "demo-button"}
          onClick={onClick}
        >
          {props.panelId}
        </button>
      </div>
      <div className="flex">
        <button
          className="demo-icon-button"
          onClick={() => {
            const panel = props.api.getPanel(props.panelId);
            if (panel) {
              props.api.addFloatingGroup(panel);
            }
          }}
        >
          <span className="material-symbols-outlined">ad_group</span>
        </button>
        <button
          className="demo-icon-button"
          onClick={() => {
            const panel = props.api.getPanel(props.panelId);
            if (panel) {
              props.api.addPopoutGroup(panel);
            }
          }}
        >
          <span className="material-symbols-outlined">open_in_new</span>
        </button>
        <button
          className="demo-icon-button"
          onClick={() => {
            const panel = props.api.getPanel(props.panelId);
            panel?.api.close();
          }}
        >
          <span className="material-symbols-outlined">close</span>
        </button>
        <button title="Panel visiblity cannot be edited manually." disabled={true} className="demo-icon-button">
          <span className="material-symbols-outlined">{visible ? "visibility" : "visibility_off"}</span>
        </button>
        <div>
          <button className="demo-icon-button" onClick={togglePopup}>
            <span className="material-symbols-outlined">edit</span>
          </button>
          {isPopupOpen && panel && <TitleEditPopup panel={panel} onClose={togglePopup} />}
        </div>
      </div>
    </div>
  );
};

const TitleEditPopup: React.FC<{ panel: IDockviewPanel; onClose: () => void }> = ({ panel, onClose }) => {
  const [title, setTitle] = useState<string>(panel.title ?? "");

  const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setTitle(event.target.value);
  };

  const onClick = () => {
    panel.setTitle(title);
    onClose();
  };

  return (
    <div className="absolute top-1/2 left-1/2 z-50 -translate-x-1/2 -translate-y-1/2 transform bg-black p-5">
      <div>
        <span className="!text-white">Edit Panel Title</span>
      </div>
      <input className="!text-black" value={title} onChange={onChange} />
      <div className="button-group">
        <button className="panel-builder-button" onClick={onClick}>
          Edit
        </button>
        <button className="panel-builder-button" onClick={onClose}>
          Close
        </button>
      </div>
    </div>
  );
};

export const PanelActions = (props: { panels: string[]; api: DockviewApi; activePanel?: string }) => {
  return (
    <div className="action-container">
      {props.panels.map((id) => {
        return <PanelAction key={id} {...props} panelId={id} />;
      })}
    </div>
  );
};
