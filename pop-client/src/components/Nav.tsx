import { useRef, useState } from "react";

import { NavLinks } from "./Links";

const useNavigation = () => {};

export const Navigation = () => {
  const [displayState, setDisplayState] = useState(false);

  return (
    <nav className="nav-container">
      <NavLinks />
    </nav>
  );
};
