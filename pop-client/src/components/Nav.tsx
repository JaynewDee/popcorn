import { useRef, useState } from "react";
import { createPortal } from "react-dom";
import { Link } from "react-router-dom";
import { NavLinks } from "./Links";

const useNavigation = () => {};

export const Navigation = () => {
  const [displayState, setDisplayState] = useState(false);

  return (
    <div className="nav-container">
      <nav>
        <NavLinks />
      </nav>
    </div>
  );
};
