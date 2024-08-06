import { twMerge } from "tailwind-merge";

interface MacOsFullScreenIconProps {
  className?: string;
}

export const MacOsFullScreenIcon = ({ className }: MacOsFullScreenIconProps) => {
  return (
    <svg
      className={twMerge("flex items-center fill-current", className)}
      viewBox="0 0 12 12"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M9 8C9 8.55228 8.55228 9 8 9H4.5L9 4.5V8Z" />
      <path d="M3 4C3 3.44772 3.44772 3 4 3L7.5 3L3 7.5L3 4Z" />
    </svg>
  );
};

export default MacOsFullScreenIcon;
