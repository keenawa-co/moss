export class Theme {
  constructor(
    public name: string,
    public type: string,
    public isDefault: boolean,
    public colors: Colors
  ) {}
}

export class Colors {
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
  ) {}
}
