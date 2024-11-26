import "@typespec/json-schema";

using TypeSpec.JsonSchema;

@jsonSchema
namespace Schemas;

model Theme {
  "name": string,
  "slug": string,
  "type": string,
  "isDefault": boolean,
  "colors": Colors
}

model Colors {
  "primary": ColorType
}

union ColorType {
  {"$type": "solid", "$value": string},
  {"$type": "gradient", "$value": ColorGradient}
}

model ColorGradient {
  "angle": integer,
  "stops": ColorGradientStop
}

model ColorGradientStop {
  "color": string,
  "position": numeric
}
