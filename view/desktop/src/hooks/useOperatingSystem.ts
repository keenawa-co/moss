export type OperatingSystem = "browser" | "linux" | "macOS" | "windows" | "unknown";

export function useOperatingSystem(): OperatingSystem {
  let os: OperatingSystem = "unknown";
  if (navigator.userAgent.indexOf("Win") != -1) os = "windows";
  if (navigator.userAgent.indexOf("Mac") != -1) os = "macOS";
  if (navigator.userAgent.indexOf("X11") != -1 || navigator.userAgent.indexOf("Linux") != -1) os = "linux";
  return os;
}
