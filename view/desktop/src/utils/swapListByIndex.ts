export const swapListByIndex = <T>(fromIndex: number, toIndex: number, list: T[]) => {
  const updatedItems = [...list];
  [updatedItems[fromIndex], updatedItems[toIndex]] = [updatedItems[toIndex], updatedItems[fromIndex]];
  return updatedItems;
};
