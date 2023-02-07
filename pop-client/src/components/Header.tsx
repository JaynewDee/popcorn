import React from "react";
import Login from "../pages/Login";
import { Navigation } from "./Nav";

const Header = () => {
  return (
    <header>
      <h1 className="app-title">POPCORN</h1>
      <Navigation />
    </header>
  );
};

export default Header;
