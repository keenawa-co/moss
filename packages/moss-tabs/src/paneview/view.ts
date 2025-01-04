import React from "react";

import { PanelUpdateEvent } from "../panel/types";
import { ReactPart, ReactPortalStore } from "../react";
import { IPaneBodyPart, PanePanelComponentInitParameter } from "./paneviewPanel";
import { IPaneviewPanelProps } from "./paneviewReact";

export class PanePanelSection implements IPaneBodyPart {
  private readonly _element: HTMLElement;
  private part?: ReactPart<IPaneviewPanelProps>;

  get element() {
    return this._element;
  }

  constructor(
    public readonly id: string,
    private readonly component: React.FunctionComponent<IPaneviewPanelProps>,
    private readonly reactPortalStore: ReactPortalStore
  ) {
    this._element = document.createElement("div");
    this._element.className = "h-full w-full";
  }

  public init(parameters: PanePanelComponentInitParameter): void {
    this.part = new ReactPart(this.element, this.reactPortalStore, this.component, {
      params: parameters.params,
      api: parameters.api,
      title: parameters.title,
      containerApi: parameters.containerApi,
    });
  }

  public toJSON() {
    return {
      id: this.id,
    };
  }

  public update(params: PanelUpdateEvent) {
    this.part?.update(params.params);
  }

  public dispose() {
    this.part?.dispose();
  }
}
