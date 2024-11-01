import type { Window } from "@tauri-apps/api/window";
import React, { createContext, useCallback, useEffect, useState } from "react";
import type { OsType } from "@tauri-apps/plugin-os";

let osType: OsType | undefined = undefined;
let osTypePromise: Promise<OsType> | null = null;

if (typeof window !== "undefined") {
  osTypePromise = import("@tauri-apps/plugin-os")
    .then((module) => {
      return module.type();
    })
    .then((x) => {
      osType = x; // Assign the value of osType here
      return x; // Return the value to the promise chain
    });
}

// A helper function to get the OS type, which returns a Promise
function getOsType(): Promise<OsType> {
  if (!osTypePromise) {
    // If the module was already loaded, just return the result
    return Promise.resolve(osType!); // Use non-null assertion
  }

  // If the module is still loading, wait for it to finish and return the result
  return osTypePromise;
}

interface TauriAppWindowContextType {
  appWindow: Window | null;
  isWindowMaximized: boolean;
  minimizeWindow: () => Promise<void>;
  maximizeWindow: () => Promise<void>;
  fullscreenWindow: () => Promise<void>;
  closeWindow: () => Promise<void>;
}

export const TauriAppWindowContext = createContext<TauriAppWindowContextType>({
  appWindow: null,
  isWindowMaximized: false,
  minimizeWindow: () => Promise.resolve(),
  maximizeWindow: () => Promise.resolve(),
  fullscreenWindow: () => Promise.resolve(),
  closeWindow: () => Promise.resolve(),
});

interface TauriAppWindowProviderProps {
  children: React.ReactNode;
}

export const TauriAppWindowProvider: React.FC<TauriAppWindowProviderProps> = ({ children }: any) => {
  const [appWindow, setAppWindow] = useState<Window | null>(null);
  const [isWindowMaximized, setIsWindowMaximized] = useState(false);

  // Fetch the Tauri window plugin when the component mounts
  // Dynamically import plugin-window for next.js, sveltekit, nuxt etc. support:
  // https://github.com/tauri-apps/plugins-workspace/issues/217
  useEffect(() => {
    if (typeof window !== "undefined") {
      import("@tauri-apps/api").then((module) => {
        setAppWindow(module.window.getCurrentWindow());
      });
    }
  }, []);

  const updateIsWindowMaximized = useCallback(async () => {
    if (appWindow) {
      const _isWindowMaximized = await appWindow.isMaximized();
      setIsWindowMaximized(_isWindowMaximized);
    }
  }, [appWindow]);

  useEffect(() => {
    getOsType().then((osname) => {
      // temporary: https://github.com/agmmnn/tauri-controls/issues/10#issuecomment-1675884962
      if (osname !== "macos") {
        updateIsWindowMaximized();
        let unlisten: () => void = () => {};

        const listen = async () => {
          if (appWindow) {
            unlisten = await appWindow.onResized(() => {
              updateIsWindowMaximized();
            });
          }
        };
        listen();

        return () => unlisten && unlisten();
      }
    });
  }, [appWindow, updateIsWindowMaximized]);

  const minimizeWindow = async () => {
    if (appWindow) {
      await appWindow.minimize();
    }
  };

  const maximizeWindow = async () => {
    if (appWindow) {
      await appWindow.toggleMaximize();
    }
  };

  const fullscreenWindow = async () => {
    if (appWindow) {
      const fullscreen = await appWindow.isFullscreen();
      if (fullscreen) {
        await appWindow.setFullscreen(false);
      } else {
        await appWindow.setFullscreen(true);
      }
    }
  };

  const closeWindow = async () => {
    if (appWindow) {
      await appWindow.close();
    }
  };

  return (
    <TauriAppWindowContext.Provider
      value={{
        appWindow,
        isWindowMaximized,
        minimizeWindow,
        maximizeWindow,
        fullscreenWindow,
        closeWindow,
      }}
    >
      {children}
    </TauriAppWindowContext.Provider>
  );
};

export default TauriAppWindowContext;
