export class Theme {
  [key: string]: string | boolean | Colors;

  constructor(
    public name: string,
    public type: string,
    public isDefault: boolean,
    public colors: Colors
  ) {
    this["name"] = name;
    this["type"] = type;
    this["isDefault"] = isDefault;
    this["colors"] = colors;
  }
}

export class Colors {
  [key: string]: string | undefined;

  constructor(
    public primary?: string,
    public sideBarBackground?: string,
    public toolBarBackground?: string,
    public pageBackground?: string,
    public statusBarBackground?: string,
    public windowsCloseButtonBackground?: string,
    public windowControlsLinuxBackground?: string,
    public windowControlsLinuxText?: string,
    public windowControlsLinuxHoverBackground?: string,
    public windowControlsLinuxActiveBackground?: string
  ) {
    this["primary"] = primary;
    this["sideBar.background"] = sideBarBackground;
    this["toolBar.background"] = toolBarBackground;
    this["page.background"] = pageBackground;
    this["statusBar.background"] = statusBarBackground;
    this["windowsCloseButton.background"] = windowsCloseButtonBackground;
    this["windowControlsLinux.background"] = windowControlsLinuxBackground;
    this["windowControlsLinux.text"] = windowControlsLinuxText;
    this["windowControlsLinux.hoverBackground"] = windowControlsLinuxHoverBackground;
    this["windowControlsLinux.activeBackground"] = windowControlsLinuxActiveBackground;
  }
}
