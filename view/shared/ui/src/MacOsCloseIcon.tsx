import { twMerge } from "tailwind-merge";

interface MacOsCloseIconProps {
  className?: string;
}

export const MacOsCloseIcon = ({ className }: MacOsCloseIconProps) => {
  return (
    <svg
      className={twMerge("flex items-center fill-current", className)}
      viewBox="0 0 12 12"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M3.05806 8.94194C3.30214 9.18602 3.69786 9.18602 3.94194 8.94194L8.94194 3.94194C9.18602 3.69786 9.18602 3.30214 8.94194 3.05806C8.69786 2.81398 8.30214 2.81398 8.05806 3.05806L3.05806 8.05806C2.81398 8.30214 2.81398 8.69786 3.05806 8.94194Z"
      />
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M8.94194 8.94194C8.69786 9.18602 8.30214 9.18602 8.05806 8.94194L3.05806 3.94194C2.81398 3.69786 2.81398 3.30214 3.05806 3.05806C3.30214 2.81398 3.69786 2.81398 3.94194 3.05806L8.94194 8.05806C9.18602 8.30214 9.18602 8.69786 8.94194 8.94194Z"
      />
    </svg>
  );
};

export default MacOsCloseIcon;
