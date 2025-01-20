import React from "react";

import { IDockviewPanelProps } from "@repo/moss-tabs";

export const components = {
  Default: (props: IDockviewPanelProps<{ myValue: string }>) => {
    const [title, setTitle] = React.useState<string>(props.api.title ?? "");

    const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      setTitle(event.target.value);
    };

    const onClick = () => {
      props.api.setTitle(title);
    };

    return (
      <div style={{ padding: "20px", color: "white" }}>
        <div>
          <span style={{ color: "grey" }}>{"props.api.title="}</span>
          <span>{`${props.api.title}`}</span>
        </div>
        <input value={title} onChange={onChange} />
        <button onClick={onClick}>Change</button>
        {JSON.stringify(Object.keys(props.params))}
      </div>
    );
  },
};
