import { ComponentPropsWithRef } from "react";
import { cn } from "./utils/utils";

const convertToRgba = ([r, g, b, a]: [number, number, number, number]) => `rgba(${r}, ${g}, ${b}, ${a / 100})`;

export interface BadgeProps {
  style?: "full" | "compact";
  color: [[number, number, number, number], [number, number, number, number]];
  value?: string;
}

export const Badge = ({
  style = "full",
  color = [
    [0, 0, 0, 100],
    [240, 240, 240, 100],
  ],
  value = "Text",
  className,
  ...props
}: BadgeProps & Omit<ComponentPropsWithRef<"div">, "color">) => {
  const backgroundColor = convertToRgba(color[1]);
  const primaryColor = convertToRgba(color[0]);

  if (style === "compact") {
    return (
      <div
        style={{ backgroundColor: primaryColor }}
        className={cn("inline-block size-2 rounded-lg", className)}
        {...props}
      />
    );
  }

  return (
    <div
      style={{ backgroundColor, color: primaryColor }}
      className={cn("inline-block rounded-[5px] px-[5px] py-0.5 text-[10px] font-bold", className)}
      {...props}
    >
      {value}
    </div>
  );
};

export default Badge;
