// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * Detailed information about a color, including its type, optional gradient direction, and value.
 */
export type ColorDetail = {
  /**
   * Type of the color (solid or gradient).
   */
  type: ColorType;
  /**
   * Direction for gradients (e.g., "to right"). Optional for solid colors.
   */
  direction?: string;
  /**
   * The color value, either solid or gradient.
   */
  value: ColorValue;
};

/**
 * Represents a color position in a gradient.
 */
export type ColorPosition = {
  /**
   * Color value.
   */
  color: string;
  /**
   * Position of the color in the gradient, as a value between 0.0 and 1.0.
   */
  position: number;
};

/**
 * Represents the type of a color, either solid or gradient.
 */
export type ColorType = "solid" | "gradient";

/**
 * Represents a color value, which can either be a solid color or a gradient.
 */
export type ColorValue = string | Array<ColorPosition>;

/**
 * Represents a theme with properties such as name, type, default status, and color tokens.
 */
export type Theme = {
  /**
   * Display name of the theme.
   */
  name: string;
  /**
   * Slug identifier for the theme, used in file paths or URLs.
   */
  slug: string;
  /**
   * Type of the theme (light or dark).
   */
  type: ThemeType;
  /**
   * Indicates if this is the default theme.
   */
  isDefault: boolean;
  /**
   * A collection of color tokens used by the theme.
   */
  colors: { [key in string]?: ColorDetail };
};

/**
 * Represents the type of a theme, either light or dark.
 */
export type ThemeType = "light" | "dark";
