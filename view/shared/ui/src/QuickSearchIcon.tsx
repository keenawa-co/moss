import { twMerge } from "tailwind-merge";

interface QuickSearchIconProps {
  className?: string;
}

export const QuickSearchIcon = ({ className }: QuickSearchIconProps) => {
  return (
    <svg
      className={twMerge("flex items-center fill-none", className)}
      viewBox="0 0 20 20"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M12.4993 2.29102H7.49935C3.88823 2.29102 2.29102 4.16602 2.29102 8.33268V12.4993C2.29102 15.8327 3.88823 17.7077 7.49935 17.7077H12.4993C15.5549 17.7077 17.7077 15.8327 17.7077 12.4993V8.33268C17.7077 4.16602 15.5549 2.29102 12.4993 2.29102Z"
        strokeWidth="1.25"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <path d="M7.91602 14.1673L12.0827 5.83398" strokeWidth="1.25" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
};

export default QuickSearchIcon;
