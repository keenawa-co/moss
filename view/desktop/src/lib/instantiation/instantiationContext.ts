import React from "react";
import { IInstantiationService } from "./instantiation";

export const InstantiationContext = React.createContext<IInstantiationService | null>(null);
