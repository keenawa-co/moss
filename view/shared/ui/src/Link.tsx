import { cva } from "class-variance-authority";
import Icon from "./Icon";

interface Link {
  label?: string;
  url: string;
  withIcon?: boolean;
  type?: "primary" | "secondary" | "disabled";
  className?: string;
}

const linkVariants = cva("inline-flex items-center duration-300 transition-colors  ", {
  variants: {
    type: {
      primary: "text-[#0f62fe] hover:text-[#054ada]",
      secondary: "text-[#88ADF7] hover:text-[#054ada]",
      disabled: "text-[#c4c4c4] cursor-not-allowed",
    },
  },
  defaultVariants: {
    type: "primary",
  },
});

export const Link = ({ label, url, withIcon = false, type = "primary", className }: Link) => {
  return (
    <a href={url} className={linkVariants({ type, className })}>
      {label || url} {withIcon && <Icon icon="LinkArrow" />}
    </a>
  );
};

export default Link;
