import { CSSProperties } from "react";

export interface ButtonStyleProps extends Background, Border, Spacing, Typography, Effects {}

interface ColorsPseudoClasses {
  default: string;
  hover?: string;
  active?: string;
  focus?: string;
  focusWithin?: string;
  focusVisible?: string;
  visited?: string;
  target?: string;
  dataState?: "checked" | "unchecked" | "intermediate";
}

interface Background {
  background: ColorsPseudoClasses;
  backgroundAttachment?: string;
  backgroundClip?: string;
  backgroundImage?: string;
  backgroundOrigin?: string;
  backgroundPosition?: string;
  backgroundRepeat?: string;
  backgroundSize?: number;
}

interface Border {
  borderRadius?: CSSProperties["borderRadius"];
  borderWidth?: number;
  borderColor?: ColorsPseudoClasses;
  borderStyle?: CSSProperties["borderStyle"];

  outlineWidth?: string;
  outlineColor?: ColorsPseudoClasses;
  outlineStyle?: string;
  outlineOffset?: string;
}

interface Spacing {
  margin?: string;
  padding?: string;
}

interface Typography {
  fontFamily?: string;
  fontSize?: string;
  fontSmoothing?: string;
  fontStyle?: string;
  fontWeight?: string;
  fontStretch?: string;
  fontVariantNumeric?: string;
  letterSpacing?: string;
  lineClamp?: string;
  lineHeight?: string;
  listStyleImage?: string;
  listStylePosition?: string;
  listStyleType?: string;
  textAlign?: string;
  color?: ColorsPseudoClasses;
  textDecorationLine?: string;
  textDecorationColor?: string;
  textDecorationStyle?: string;
  textDecorationThickness?: string;
  textUnderlineOffset?: string;
  textTransform?: string;
  textOverflow?: string;
  textWrap?: string;
  textIndent?: string;
  verticalAlign?: string;
  whiteSpace?: string;
  wordBreak?: string;
  hyphens?: string;
  content?: string;
}

interface Effects {
  boxShadow?: string;
  opacity?: string;
  mixBlendMode?: string;
  backgroundBlendMode?: string;
}
