import { getHash, createFolder } from "addon:demo/host-functions";

export const execute = () => {
  createFolder(getHash());
};
