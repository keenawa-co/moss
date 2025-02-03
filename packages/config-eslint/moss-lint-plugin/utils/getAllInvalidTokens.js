export const getAllInvalidTokens = (str, loc, regex) => {
  const invalidTokens = [];
  let arr;

  while ((arr = regex.exec(str)) !== null) {
    const className = arr[0];
    const name = arr[1] || arr[2] || arr[3] || arr[4];

    const startColumn = loc.start.column + str.indexOf(className) + 1;
    const endColumn = startColumn + className.length;

    invalidTokens.push({
      name,
      value: className,
      loc: {
        start: {
          line: loc.start.line,
          column: startColumn,
        },
        end: {
          line: loc.end.line,
          column: endColumn,
        },
      },
    });
  }

  return invalidTokens;
};
