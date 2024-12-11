export type ColorStop = [color: string, percentage: number];

export function clamp_rgb(value: number) {
  if (value < 0) {
    return 0;
  } else if (value > 255) {
    return 255;
  } else {
    return value;
  }
}

export function clamp_alpha(value: number) {
  if (value < 0) {
    return 0;
  } else if (value > 1) {
    return 1;
  } else {
    return value;
  }
}

export function clamp_percent(percent: number) {
  if (percent < 0) {
    return 0;
  } else if (percent > 100) {
    return 100;
  } else {
    return percent;
  }
}

export function rgba(r: number, g: number, b: number, a: number) {
  return `rgba(${clamp_rgb(r)}, ${clamp_rgb(g)}, ${clamp_rgb(b)}, ${clamp_alpha(a)})`;
}

export function linearGradient(direction: string, ...colorStopList: ColorStop[]) {
  return (
    `linear-gradient(${direction}` +
    colorStopList
      .map((stop) => {
        return `, ${stop[0]} ${clamp_percent(stop[1])}%`;
      })
      .join("") +
    ")"
  );
}
