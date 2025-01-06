import { useEffect, useState } from "react";

export const usePrepareWindow = () => {
  const [isPreparing, setIsPreparing] = useState(true);

  useEffect(() => {
    setIsPreparing(false);
  }, []);

  return { isPreparing };
};
