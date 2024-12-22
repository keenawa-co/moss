import { Parameters } from "@repo/moss-tabs-core";

export interface PanelParameters<T extends {} = Parameters> {
  params: T;
}
