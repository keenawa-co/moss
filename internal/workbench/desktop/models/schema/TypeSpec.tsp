import "@typespec/json-schema";

using TypeSpec.JsonSchema;

@jsonSchema
namespace Schemas {
  model Theme {
    "name": string,
    "slug": string,
    "type": string,
    "isDefault": boolean,
    "color": Colors
  }
}

model Colors {
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
scalar ColorValue extends string;

union ColorType {
  {
    "$type": "solid",
    "$value": ColorValue
  },
  {
    "$type": "gradient",
    "$direction": string,
    "$value": ColorGradientEntry[]
  }
}

model ColorGradientEntry {
  "color": ColorValue,

  @minValue(0.0)
  @maxValue(1.0)
  "position": numeric
}
