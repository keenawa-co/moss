interface Identifiable {
  id: number;
}

export const swapListById = <T extends Identifiable>(fromId: number, toId: number, list: T[]) => {
  const fromIndex = list.findIndex((item) => item.id === fromId);
  const toIndex = list.findIndex((item) => item.id === toId);

  if (fromIndex === -1 || toIndex === -1) {
    return null;
  }

  const updatedItems = [...list];
  [updatedItems[fromIndex], updatedItems[toIndex]] = [updatedItems[toIndex], updatedItems[fromIndex]];

  return updatedItems;
};
