import "@typespec/json-schema";

using TypeSpec.JsonSchema;



@jsonSchema
namespace Schemas {
    model Configuration {
      @minValue(800)
      @maxValue(3840)
      @doc("The width of the application window in pixels.")
      @contentMediaType("APPLICATION")
      "window.defaultWidth"?: integer = 800;

      @minValue(600)
      @maxValue(2160)
      @doc("The height of the application window in pixels.")
      @summary("Application")
      @contentMediaType("APPLICATION")
      "window.defaultHeight"?: integer = 600;

      "tes": SolidColor = #{
        type: "222",
        value: 12,
      };
    }
}

model SolidColor {
  type: string;
  value: int16;
}