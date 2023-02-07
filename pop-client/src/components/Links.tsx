import { Link } from "react-router-dom";

export const ToHome = () => (
  <Link to={"/"} className="nav-link">
    home
  </Link>
);

export const ToLearn = () => (
  <Link to={"/learn"} className="nav-link">
    learn
  </Link>
);

export const ToCreate = () => (
  <Link to={"/create"} className="nav-link">
    create
  </Link>
);

export const ToStudy = () => (
  <Link to={"/study"} className="nav-link">
    study
  </Link>
);

export const ToLogin = () => (
  <Link to={"/login"} className="nav-link">
    login
  </Link>
);

const Links = [ToHome, ToLearn, ToCreate, ToStudy, ToLogin];

export const NavLinks = () => {
  return <>{Links.map((fn) => fn())}</>;
};
