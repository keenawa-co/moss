import { Theme, Colors } from "@repo/moss-models";

// Utility function to convert camelCase to kebab-case
function toKebabCase(str: string): string {
  return str
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .toLowerCase()
    .replace(/\./g, "-");
}

// Type to convert string keys to CSS variable format
export type ThemeCssVariables = {
  [K in keyof Colors as `--color-${KebabCase<K>}`]: string;
};

// Mapped type to convert string to kebab-case using Template Literal Types
type KebabCase<S extends string> = S extends `${infer T}${infer U}`
  ? U extends Uncapitalize<U>
    ? `${Lowercase<T>}${KebabCase<U>}`
    : `${Lowercase<T>}-${KebabCase<U>}`
  : S;

// Function to map Theme colors to CSS variables
export function mapThemeToCssVariables(theme: Theme): ThemeCssVariables {
  const cssVariables = {} as ThemeCssVariables;

  // Dynamically iterate over the keys of the colors object
  Object.entries(theme.colors).forEach(([key, value]) => {
    const cssKey = `--color-${toKebabCase(key)}` as keyof ThemeCssVariables;
    cssVariables[cssKey] = value;
  });

  return cssVariables;
}

// Function to generate Tailwind CSS color variables with opacity
export const customTailwindColorVariables: Record<keyof Colors, string> = Object.keys({} as Colors).reduce(
  (acc, key) => {
    const cssKey = `--color-${toKebabCase(key)}` as keyof ThemeCssVariables;
    acc[key as keyof Colors] = `rgba(var(${cssKey}), var(--tw-bg-opacity, 1))`;
    return acc;
  },
  {} as Record<keyof Colors, string>
);

// Utility function to handle RGBA with opacity
function rgbaWithOpacity(variableName: keyof ThemeCssVariables): string {
  return `rgba(var(${variableName}), var(--tw-bg-opacity, 1))`;
}

// Conversion class using JSON parsing with type safety
export class Convert {
  public static toTheme(json: string): Theme {
    return cast(JSON.parse(json), themeType);
  }

  public static themeToJson(value: Theme): string {
    return JSON.stringify(uncast(value, themeType), null, 2);
  }
}

// Define the type schema for Theme and Colors
const themeType: Type = {
  type: "object",
  properties: {
    name: { type: "string" },
    slug: { type: "string" },
    type: { type: "string" },
    isDefault: { type: "boolean" },
    colors: {
      type: "object",
      additionalProperties: { type: "string" },
    },
  },
  required: ["name", "slug", "type", "isDefault", "colors"],
};

type Type =
  | { type: "string" }
  | { type: "number" }
  | { type: "boolean" }
  | { type: "object"; properties?: Record<string, Type>; additionalProperties?: Type; required?: string[] }
  | { type: "array"; items: Type }
  | { type: "union"; types: Type[] };

// Function to cast JSON to TypeScript types with validation
function cast<T>(val: any, typ: Type): T {
  return transform(val, typ, []);
}

// Function to uncast TypeScript types to JSON with validation
function uncast<T>(val: T, typ: Type): any {
  return transform(val, typ, []);
}

function transform(val: any, typ: Type, stack: string[] = []): any {
  switch (typ.type) {
    case "string":
      return validateString(val, stack);
    case "number":
      return validateNumber(val, stack);
    case "boolean":
      return validateBoolean(val, stack);
    case "object":
      return validateObject(val, typ, stack);
    case "array":
      return validateArray(val, typ, stack);
    case "union":
      return validateUnion(val, typ, stack);
    default:
      throw new Error(`Unknown type '${(typ as any).type}' at ${formatStack(stack)}`);
  }
}

// Helper function to format the stack for error messages
function formatStack(stack: string[]): string {
  return stack.length ? stack.join(".") : "root";
}

function validateString(val: any, stack: string[]): string {
  if (typeof val !== "string") {
    throw new Error(`Expected string but got ${typeof val} at ${formatStack(stack)}`);
  }
  return val;
}

function validateNumber(val: any, stack: string[]): number {
  if (typeof val !== "number") {
    throw new Error(`Expected number but got ${typeof val} at ${formatStack(stack)}`);
  }
  return val;
}

function validateBoolean(val: any, stack: string[]): boolean {
  if (typeof val !== "boolean") {
    throw new Error(`Expected boolean but got ${typeof val} at ${formatStack(stack)}`);
  }
  return val;
}

function validateObject(val: any, typ: Type, stack: string[]): any {
  if (typeof val !== "object" || val === null || Array.isArray(val)) {
    throw new Error(`Expected object but got ${getType(val)} at ${formatStack(stack)}`);
  }

  if (typ.type !== "object") {
    throw new Error(`Expected type 'object' but got '${typ.type}' at ${formatStack(stack)}`);
  }

  const result: any = {};

  // Validate required properties
  if (typ.required) {
    for (const reqKey of typ.required) {
      if (!(reqKey in val)) {
        throw new Error(`Missing required property '${reqKey}' at ${formatStack([...stack, reqKey])}`);
      }
    }
  }

  // Validate defined properties
  if (typ.properties) {
    for (const key in typ.properties) {
      if (Object.prototype.hasOwnProperty.call(typ.properties, key)) {
        stack.push(key);
        result[key] = transform(val[key], typ.properties[key], stack);
        stack.pop();
      }
    }
  }

  // Validate additional properties
  if (typ.additionalProperties) {
    for (const key in val) {
      if (!typ.properties || !Object.prototype.hasOwnProperty.call(typ.properties, key)) {
        stack.push(key);
        result[key] = transform(val[key], typ.additionalProperties, stack);
        stack.pop();
      }
    }
  }

  return result;
}

function validateArray(val: any, typ: Type, stack: string[]): any[] {
  if (typ.type !== "array") {
    throw new Error(`Expected type 'array' but got '${typ.type}' at ${formatStack(stack)}`);
  }

  if (!Array.isArray(val)) {
    throw new Error(`Expected array but got ${getType(val)} at ${formatStack(stack)}`);
  }

  return val.map((item, index) => transform(item, typ.items, [...stack, String(index)]));
}

function validateUnion(val: any, typ: Type, stack: string[]): any {
  if (typ.type !== "union") {
    throw new Error(`Expected type 'union' but got '${typ.type}' at ${formatStack(stack)}`);
  }

  const errors: string[] = [];

  for (const subtype of typ.types) {
    try {
      return transform(val, subtype, stack);
    } catch (e) {
      if (e instanceof Error) {
        errors.push(e.message);
      }
    }
  }

  throw new Error(`Value does not match any type in union: ${errors.join(" | ")} at ${formatStack(stack)}`);
}

function getType(val: any): string {
  if (val === null) return "null";
  if (Array.isArray(val)) return "array";
  return typeof val;
}
