import { useState } from "react";
import { useLocation } from "react-router-dom";
export const usePageState = () => {
  const { pathname } = useLocation();
  const state = (
    pathname
      .split("/")
      .filter((c) => c !== "/")
      .join("") + " header"
  ).toUpperCase();
  return [state];
};
