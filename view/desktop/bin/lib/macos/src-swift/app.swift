import AppKit
import SwiftRs


@_cdecl("set_app_name")
public func setAppName(name: SRString) {
//   let appName = "Moss Compass"
  DispatchQueue.main.async {
    if let mainWindow = NSApp.mainWindow {
      mainWindow.title = name.toString()
    } else {
      setAppName(name: name)
    }

    if let mainMenu = NSApp.mainMenu {
      if let appMenuItem = mainMenu.items.first {
        appMenuItem.title = name.toString()
      }
    }
  }
}