import { twMerge } from "tailwind-merge";
import { ComponentPropsWithoutRef } from "react";
import { Icon, SearchIcon, IconTitle } from ".";

export const TempSearchComponent = ({
  children,
  title,
  className,
}: { title?: string } & ComponentPropsWithoutRef<"div">) => {
  return (
    <div
      className={twMerge(
        "flex items-center gap-2.5 hover:bg-stone-500 hover:bg-opacity-10 focus:bg-stone-500 focus:bg-opacity-10 rounded-lg transition ml-3.5 w-57 h-8 pl-2.5",
        className
      )}
    >
      {children}
      <TempSearchComponent className="group">
        <Icon className="h-4.5 w-4.5">
          <SearchIcon className="text-stone-500 hover:text-stone-600" />
        </Icon>
        <IconTitle className="text-stone-900" title="Search..." />
      </TempSearchComponent>
    </div>
  );
};

export default TempSearchComponent;
