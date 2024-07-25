import { twMerge } from 'tailwind-merge'

interface SearchIconProps {
  className?: string
}

export const SearchIcon = ({ className }: SearchIconProps) => {
  return (
    <svg
      className={twMerge('flex items-center fill-current', className)}
      viewBox="0 0 16 16"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M8.09883 3.15078C5.36502 3.15078 3.14883 5.36697 3.14883 8.10078C3.14883 10.8346 5.36502 13.0508 8.09883 13.0508C10.8326 13.0508 13.0488 10.8346 13.0488 8.10078C13.0488 5.36697 10.8326 3.15078 8.09883 3.15078ZM1.79883 8.10078C1.79883 4.62139 4.61943 1.80078 8.09883 1.80078C11.5782 1.80078 14.3988 4.62139 14.3988 8.10078C14.3988 11.5802 11.5782 14.4008 8.09883 14.4008C4.61943 14.4008 1.79883 11.5802 1.79883 8.10078Z"
      />
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M11.6 11.6C11.8637 11.3364 12.291 11.3364 12.5546 11.6L16.0021 15.0476C16.2657 15.3112 16.2657 15.7386 16.0021 16.0022C15.7385 16.2658 15.3111 16.2658 15.0475 16.0022L11.6 12.5546C11.3364 12.291 11.3364 11.8636 11.6 11.6Z"
      />
    </svg>
  )
}

export default SearchIcon
