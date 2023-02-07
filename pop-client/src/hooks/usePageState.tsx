import { useState } from "react";
import { useLocation } from "react-router-dom";
export const usePageState = () => {
  const { pathname } = useLocation();
  return (
    pathname
      .split("/")
      .filter((c) => c !== "/")
      .join("") + " header"
  ).toUpperCase();
};
