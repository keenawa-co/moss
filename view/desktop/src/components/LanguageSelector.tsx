import { LANGUAGES } from "@/constants";
import { RootState } from "@/store";
import { setLanguage } from "@/store/languages/languagesSlice";
import React from "react";
import { useDispatch, useSelector } from "react-redux";

export const LanguageSelector = () => {
  const dispatch = useDispatch();
  const language = useSelector((state: RootState) => state.languages.code);

  const onChangeLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    console.log("Language changed", e.target.value);
    dispatch(setLanguage(e.target.value));
  };

  return (
    <select className="bg-purple-300 text-[rgba(var(--color-primary))]" value={language} onChange={onChangeLang}>
      {LANGUAGES.map(({ code, label }) => (
        <option key={code} value={code}>
          {label}
        </option>
      ))}
    </select>
  );
};

export default LanguageSelector;
