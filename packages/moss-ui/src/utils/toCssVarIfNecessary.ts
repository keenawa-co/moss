export const toCssVarIfNecessary = (value?: string) => {
  if (value && value.startsWith("--")) {
    return `var(${value})`;
  }

  return value;
};
