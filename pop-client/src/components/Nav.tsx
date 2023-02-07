import { useRef, useState } from "react";
import { createPortal } from "react-dom";
import { Link } from "react-router-dom";

const useNavigation = () => {};

export const Navigation = () => {
  const [displayState, setDisplayState] = useState(false);

  return (
    <div
      className={
        "nav-container " +
        (displayState ? "nav-container-expanded" : "nav-container-collapsed")
      }
    >
      {/* <button onClick={() => setDisplayState((prev) => !prev)}>\/</button> */}
      <nav>
        <Link to={"/"} className="nav-link">
          home
        </Link>
        <Link to={"/learn"} className="nav-link">
          learn
        </Link>
        <Link to={"/create"} className="nav-link">
          create
        </Link>
        <Link to={"/study"} className="nav-link">
          study
        </Link>
        <Link to={"/login"} className="nav-link">
          login
        </Link>
      </nav>
    </div>
  );
};
