import { Scope } from "@radix-ui/react-context";

export type ScopedProps<P> = P & { __scopeContextMenu?: Scope; __scopeDropdownMenu?: Scope; __scopePopover?: Scope };
