export const toCssVarIfNecessary = (value: string) => {
  if (value.startsWith("--")) {
    return `var(${value})`;
  }

  return value;
};
