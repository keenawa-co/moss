import { useEffect, useState } from "react";
import { useAppDispatch } from "../store";
import { setLanguageFromLocalStorage } from "../store/languages/languagesSlice";

export const usePrepareWindow = () => {
  const dispatch = useAppDispatch();
  const [isPreparing, setIsPreparing] = useState(true);

  useEffect(() => {
    // Dispatch Redux actions
    dispatch(setLanguageFromLocalStorage());

    setIsPreparing(false);
  }, [dispatch]);

  return { isPreparing };
};
