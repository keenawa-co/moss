import { twMerge } from "tailwind-merge";

interface GoalsIconProps {
  className?: string;
}

export const GoalsIcon = ({ className }: GoalsIconProps) => {
  return (
    <svg
      className={twMerge("flex items-center", className)}
      viewBox="0 0 16 16"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M8.875 1V3.7M8.65 13.6V16.3M1 8.65H3.7M13.6 8.65H16.3M8.65 8.9875V8.3125M14.95 8.65C14.95 12.1294 12.1294 14.95 8.65 14.95C5.17061 14.95 2.35 12.1294 2.35 8.65C2.35 5.17061 5.17061 2.35 8.65 2.35C12.1294 2.35 14.95 5.17061 14.95 8.65ZM7.975 8.65C7.975 8.27721 8.27721 7.975 8.65 7.975C9.02279 7.975 9.325 8.27721 9.325 8.65C9.325 9.02279 9.02279 9.325 8.65 9.325C8.27721 9.325 7.975 9.02279 7.975 8.65Z"
        strokeWidth="1.3"
        strokeLinecap="round"
      />
    </svg>
  );
};

export default GoalsIcon;
