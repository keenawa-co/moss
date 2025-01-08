export type integer = any;

export type SolidColor = {
  type: string;
  value: number;
};
export namespace Schemas {
  export type Configuration = {
    /**
     * The width of the application window in pixels.
     */
    defaultWidth?: integer;
    /**
     * The height of the application window in pixels.
     */
    defaultHeight?: integer;
    tes: SolidColor;
  };
}
