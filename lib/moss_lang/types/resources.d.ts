interface Resources {
  ns1: {
    settings: "Settings";
    home: "Home";
    title: "Moss Studio";
    selectTheme: "Select theme:";
    selectLanguage: "Select language:";
    user: "My name is: {{name}}";
    description: {
      part1: "Welcome to react using <1>react-i18next</1> fully type-safe.";
      part2: "😉";
    };
  };
  ns2: {
    description: {
      part1: "In order to infer the appropriate type for t function, you should use type augmentation to override the Resources type.";
      part2: "Check out the @types/i18next to see an example.";
    };
  };
}

export default Resources;
