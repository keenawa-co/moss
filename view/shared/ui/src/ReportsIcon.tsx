import { twMerge } from "tailwind-merge";

interface ReportsIconProps {
  className?: string;
}

export const ReportsIcon = ({ className }: ReportsIconProps) => {
  return (
    <svg
      className={twMerge("flex items-center fill-current", className)}
      viewBox="0 0 16 16"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M6.07578 12.1754C5.7168 12.1754 5.42578 12.4664 5.42578 12.8254C5.42578 13.1844 5.7168 13.4754 6.07578 13.4754V12.1754ZM9.22578 13.4754C9.58477 13.4754 9.87578 13.1844 9.87578 12.8254C9.87578 12.4664 9.58477 12.1754 9.22578 12.1754V13.4754ZM6.07578 9.47539C5.7168 9.47539 5.42578 9.76641 5.42578 10.1254C5.42578 10.4844 5.7168 10.7754 6.07578 10.7754V9.47539ZM11.0258 10.7754C11.3848 10.7754 11.6758 10.4844 11.6758 10.1254C11.6758 9.76641 11.3848 9.47539 11.0258 9.47539V10.7754ZM11.2084 2.65804L11.668 2.19842L11.668 2.19842L11.2084 2.65804ZM15.0385 6.76943L15.6705 6.61769L15.6705 6.61769L15.0385 6.76943ZM14.8768 6.37921L15.4311 6.03958L15.4311 6.03958L14.8768 6.37921ZM3.26911 14.2555L2.68996 14.5506H2.68996L3.26911 14.2555ZM4.64571 15.6321L4.35062 16.2112L4.64571 15.6321ZM10.3317 2.06269L10.4835 1.43065L10.3317 2.06269ZM10.722 2.22433L11.0616 1.67011L11.0616 1.67011L10.722 2.22433ZM4.64571 2.36872L4.9408 2.94787L4.64571 2.36872ZM3.26911 3.74532L3.84826 4.04041L3.26911 3.74532ZM14.4258 10.1254C14.4258 10.4844 14.7168 10.7754 15.0758 10.7754C15.4348 10.7754 15.7258 10.4844 15.7258 10.1254H14.4258ZM9.22578 16.6254C9.58477 16.6254 9.87578 16.3344 9.87578 15.9754C9.87578 15.6164 9.58477 15.3254 9.22578 15.3254V16.6254ZM6.07578 6.77539C5.7168 6.77539 5.42578 7.06641 5.42578 7.42539C5.42578 7.78438 5.7168 8.07539 6.07578 8.07539V6.77539ZM7.42578 8.07539C7.78477 8.07539 8.07578 7.78438 8.07578 7.42539C8.07578 7.06641 7.78477 6.77539 7.42578 6.77539V8.07539ZM10.0191 5.70546L10.5983 5.41037L10.5983 5.41037L10.0191 5.70546ZM11.3957 7.08206L11.6908 6.50291L11.6908 6.50291L11.3957 7.08206ZM13.0508 11.9254L13.4738 11.4319L13.0508 11.0693L12.6278 11.4319L13.0508 11.9254ZM12.4008 15.9754C12.4008 16.3344 12.6918 16.6254 13.0508 16.6254C13.4098 16.6254 13.7008 16.3344 13.7008 15.9754H12.4008ZM13.8878 13.4989C14.1603 13.7325 14.5707 13.701 14.8043 13.4284C15.0379 13.1558 15.0064 12.7455 14.7338 12.5119L13.8878 13.4989ZM11.3678 12.5119C11.0952 12.7455 11.0636 13.1558 11.2973 13.4284C11.5309 13.701 11.9412 13.7325 12.2138 13.4989L11.3678 12.5119ZM6.07578 13.4754H9.22578V12.1754H6.07578V13.4754ZM6.07578 10.7754H11.0258V9.47539H6.07578V10.7754ZM7.96578 2.67539H9.68108V1.37539H7.96578V2.67539ZM3.57578 10.9354V7.06539H2.27578V10.9354H3.57578ZM10.7488 3.11766L13.9835 6.35236L14.9028 5.43312L11.668 2.19842L10.7488 3.11766ZM15.7258 7.42009C15.7258 7.12083 15.73 6.86526 15.6705 6.61769L14.4064 6.92117C14.4216 6.98433 14.4258 7.05896 14.4258 7.42009H15.7258ZM13.9835 6.35236C14.2389 6.60772 14.2887 6.66345 14.3226 6.71883L15.4311 6.03958C15.298 5.82249 15.1144 5.64473 14.9028 5.43312L13.9835 6.35236ZM15.6705 6.61769C15.6215 6.41362 15.5407 6.21853 15.4311 6.03958L14.3226 6.71883C14.361 6.78146 14.3893 6.84974 14.4064 6.92117L15.6705 6.61769ZM2.27578 10.9354C2.27578 11.8067 2.27528 12.4994 2.32086 13.0572C2.36706 13.6227 2.46382 14.1067 2.68996 14.5506L3.84826 13.9604C3.73107 13.7304 3.65617 13.4364 3.61654 12.9514C3.57629 12.4587 3.57578 11.8282 3.57578 10.9354H2.27578ZM7.96578 15.3254C7.07297 15.3254 6.44246 15.3249 5.9498 15.2846C5.46473 15.245 5.17081 15.1701 4.9408 15.0529L4.35062 16.2112C4.79443 16.4374 5.27847 16.5341 5.84394 16.5803C6.40181 16.6259 7.09442 16.6254 7.96578 16.6254V15.3254ZM2.68996 14.5506C3.05428 15.2656 3.6356 15.8469 4.35062 16.2112L4.9408 15.0529C4.4704 14.8132 4.08795 14.4308 3.84826 13.9604L2.68996 14.5506ZM9.68108 2.67539C10.0422 2.67539 10.1168 2.67957 10.18 2.69473L10.4835 1.43065C10.2359 1.37121 9.98034 1.37539 9.68108 1.37539V2.67539ZM11.668 2.19842C11.4565 1.98682 11.2787 1.80314 11.0616 1.67011L10.3823 2.77854C10.4377 2.81248 10.4935 2.8623 10.7488 3.11766L11.668 2.19842ZM10.18 2.69473C10.2514 2.71188 10.3197 2.74016 10.3823 2.77854L11.0616 1.67011C10.8826 1.56045 10.6876 1.47964 10.4835 1.43065L10.18 2.69473ZM7.96578 1.37539C7.09442 1.37539 6.40181 1.37489 5.84394 1.42047C5.27847 1.46667 4.79443 1.56343 4.35062 1.78957L4.9408 2.94787C5.17081 2.83068 5.46473 2.75578 5.9498 2.71615C6.44246 2.6759 7.07297 2.67539 7.96578 2.67539V1.37539ZM3.57578 7.06539C3.57578 6.17258 3.57629 5.54207 3.61654 5.04941C3.65617 4.56434 3.73107 4.27042 3.84826 4.04041L2.68996 3.45023C2.46382 3.89404 2.36706 4.37808 2.32086 4.94355C2.27528 5.50142 2.27578 6.19403 2.27578 7.06539H3.57578ZM4.35062 1.78957C3.6356 2.15388 3.05428 2.73521 2.68996 3.45023L3.84826 4.04041C4.08795 3.57001 4.4704 3.18756 4.9408 2.94787L4.35062 1.78957ZM14.4258 7.42009V10.1254H15.7258V7.42009H14.4258ZM9.22578 15.3254H7.96578V16.6254H9.22578V15.3254ZM6.07578 8.07539H7.42578V6.77539H6.07578V8.07539ZM9.02578 2.02539V2.38539H10.3258V2.02539H9.02578ZM14.7158 8.07539H15.0758V6.77539H14.7158V8.07539ZM9.02578 2.38539C9.02578 3.25675 9.02528 3.94936 9.07086 4.50724C9.11706 5.0727 9.21382 5.55674 9.43996 6.00055L10.5983 5.41037C10.4811 5.18036 10.4062 4.88645 10.3665 4.40137C10.3263 3.90871 10.3258 3.2782 10.3258 2.38539H9.02578ZM14.7158 6.77539C13.823 6.77539 13.1925 6.77489 12.6998 6.73463C12.2147 6.695 11.9208 6.6201 11.6908 6.50291L11.1006 7.66122C11.5444 7.88735 12.0285 7.98412 12.5939 8.03032C13.1518 8.0759 13.8444 8.07539 14.7158 8.07539V6.77539ZM9.43996 6.00055C9.80428 6.71557 10.3856 7.2969 11.1006 7.66122L11.6908 6.50291C11.2204 6.26322 10.8379 5.88077 10.5983 5.41037L9.43996 6.00055ZM12.4008 11.9254V15.9754H13.7008V11.9254H12.4008ZM14.7338 12.5119L13.4738 11.4319L12.6278 12.4189L13.8878 13.4989L14.7338 12.5119ZM12.6278 11.4319L11.3678 12.5119L12.2138 13.4989L13.4738 12.4189L12.6278 11.4319Z" />
    </svg>
  );
};

export default ReportsIcon;