import "@typespec/json-schema";

using TypeSpec.JsonSchema;

@jsonSchema
namespace Schema {
  model Theme {
    "name": string,
    "slug": string,
    "type": string,
    "isDefault": boolean,
    "color": Color
  }
}

model Color {
  "primary": ColorType,
  "sideBar.background": ColorType,
  "toolBar.background": ColorType,
  "page.background": ColorType,
  "statusBar.background": ColorType,
  "windowsCloseButton.background": ColorType,
  "windowControlsLinux.background": ColorType,
  "windowControlsLinux.text": ColorType,
  "windowControlsLinux.hoverBackground": ColorType,
  "windowControlsLinux.activeBackground": ColorType

}


@pattern("#[0-9a-fA-F]{3,8}|rgb(a)?\\(.+\\)|hsl(a)?\\(.+\\)|[a-z]+")
scalar ColorTokenValue extends string;

union ColorType {
  {
    "$type": "solid",
    "$value": ColorTokenValue
  },
  {
    "$type": "gradient",
    "$direction": string,
    "$value": ColorGradientEntry[]
  }
}

model ColorGradientEntry {
  "color": ColorTokenValue,
  "position": numeric | string
}
