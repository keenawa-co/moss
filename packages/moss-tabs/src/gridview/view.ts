import { GridviewApi } from "../api/component.api";
import { IFrameworkPart } from "../panel/types";
import { ReactPart, ReactPortalStore } from "../react";
import { GridviewComponent } from "./gridviewComponent";
import { GridviewInitParameters, GridviewPanel } from "./gridviewPanel";
import { IGridviewPanelProps } from "./gridviewReact";

export class ReactGridPanelView extends GridviewPanel {
  constructor(
    id: string,
    component: string,
    private readonly reactComponent: React.FunctionComponent<IGridviewPanelProps>,
    private readonly reactPortalStore: ReactPortalStore
  ) {
    super(id, component);
  }

  getComponent(): IFrameworkPart {
    return new ReactPart(this.element, this.reactPortalStore, this.reactComponent, {
      params: this._params?.params ?? {},
      api: this.api,
      // TODO: fix casting hack
      containerApi: new GridviewApi((this._params as GridviewInitParameters).accessor as GridviewComponent),
    });
  }
}
