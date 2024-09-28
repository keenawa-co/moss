export function swapByIndex<T>(array: T[], index1: number, index2: number): T[] {
  if (index1 >= 0 && index1 < array.length && index2 >= 0 && index2 < array.length) {
    const newArray = [...array];
    const temp = newArray[index1];
    newArray[index1] = newArray[index2];
    newArray[index2] = temp;
    return newArray;
  } else {
    console.error(`Wrong indexes passed to swapByIndex function. index1: ${index1}, index2: ${index2}`);
  }
  return array;
}
