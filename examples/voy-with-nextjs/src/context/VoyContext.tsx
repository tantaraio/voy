import { createContext, useContext, PropsWithChildren } from "react";

import * as voy from "voy-search";

const VoyContext = createContext<typeof voy | null>(null);

export const useVoy = () => {
  const context = useContext(VoyContext);
  if (context === null) {
    throw new Error("useVoy must be used within a VoyProvider");
  }
  return context;
};

export const VoyProvider = ({ children }: PropsWithChildren) => {
  return <VoyContext.Provider value={voy}>{children}</VoyContext.Provider>;
};
