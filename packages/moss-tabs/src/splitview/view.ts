import { SplitviewApi } from "../api/component.api";
import { ReactPart, ReactPortalStore } from "../react";
import { PanelViewInitParameters } from "./options";
import { SplitviewPanel } from "./splitviewPanel";
import { ISplitviewPanelProps } from "./splitviewReact";

export class ReactPanelView extends SplitviewPanel {
  constructor(
    id: string,
    component: string,
    private readonly reactComponent: React.FunctionComponent<ISplitviewPanelProps>,
    private readonly reactPortalStore: ReactPortalStore
  ) {
    super(id, component);
  }

  getComponent(): ReactPart<ISplitviewPanelProps> {
    return new ReactPart(this.element, this.reactPortalStore, this.reactComponent, {
      params: this._params?.params ?? {},
      api: this.api,
      containerApi: new SplitviewApi((this._params as PanelViewInitParameters).accessor),
    });
  }
}
