import "@typespec/json-schema";

using TypeSpec.JsonSchema;

alias ThemeType = "light" | "dark";

@jsonSchema
namespace Schemas {
  model Theme {
    @doc("The display name of the theme. It is used to identify the theme in a human-readable format.")
    name: string;

    @doc("A unique identifier for the theme, used as a slug for referencing the theme in URLs or configuration.")
    slug: string;

    @doc("The category or type of the theme, e.g., 'dark', 'light'.")
    type: ThemeType;

    @doc("An object containing the color configuration for various UI elements.")
    colors: Colors;
  }
}

model Colors {
  "primary": ColorDefinition;
  "sideBar.background": ColorDefinition;
  "toolBar.background": ColorDefinition;
  "page.background": ColorDefinition;
  "statusBar.background": ColorDefinition;
  "windowsCloseButton.background": ColorDefinition;
  "windowControlsLinux.background": ColorDefinition;
  "windowControlsLinux.text": ColorDefinition;
  "windowControlsLinux.hoverBackground": ColorDefinition;
  "windowControlsLinux.activeBackground": ColorDefinition;
}

union ColorDefinition {
  SolidColor,
  GradientColor,
}

@pattern("#[0-9a-fA-F]{3,8}|rgb(a)?\\(.+\\)|hsl(a)?\\(.+\\)|[a-z]+")
scalar SolidString extends string;

@pattern("\\w+-gradient\\(.*?\\)\\)|\\w+-gradient\\(.*?\\)")
scalar GradientString extends string;

model SolidColor {
  type: "solid";
  value: SolidString;
}

model GradientColor {
  type: "gradient";
  value: GradientString;
}
