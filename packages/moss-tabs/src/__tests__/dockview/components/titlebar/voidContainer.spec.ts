import { beforeEach, describe, expect, test, vi } from "vitest";

import { fireEvent } from "@testing-library/dom";
import { fromPartial } from "@total-typescript/shoehorn";

import { VoidContainer } from "../../../../dockview/components/titlebar/voidContainer";
import { DockviewComponent } from "../../../../dockview/dockviewComponent";
import { DockviewGroupPanel } from "../../../../dockview/dockviewGroupPanel";

describe("voidContainer", () => {
  test("that `pointerDown` triggers activation", () => {
    const accessor = fromPartial<DockviewComponent>({
      doSetGroupActive: vi.fn(),
    });
    const group = fromPartial<DockviewGroupPanel>({});
    const cut = new VoidContainer(accessor, group);

    expect(accessor.doSetGroupActive).not.toHaveBeenCalled();

    fireEvent.pointerDown(cut.element);
    expect(accessor.doSetGroupActive).toHaveBeenCalledWith(group);
  });
});
