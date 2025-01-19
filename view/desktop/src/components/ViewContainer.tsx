import { useGetViewGroup } from "@/hooks/useViewGroups";

import * as components from "./index";

export const ViewContainer = ({ groupId }: { groupId: string }) => {
  const { data: viewGroup } = useGetViewGroup(groupId);

  if (!viewGroup) return <div>Loading...</div>;

  const Tag = components[viewGroup.component as keyof typeof components];

  if (!Tag) return <div className="flex h-full flex-col">No group view was returned</div>;

  return <Tag />;
};
