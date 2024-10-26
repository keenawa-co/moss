import { ComponentPropsWithoutRef } from "react";

export const Separator = (props: ComponentPropsWithoutRef<"div">) => {
  return <div className="my-1 h-px w-full bg-[#EBECF0]" {...props} />;
};
