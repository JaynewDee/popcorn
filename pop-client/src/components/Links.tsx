import { Link } from "react-router-dom";

const nv = "nav-link";

export const ToHome = (key: number) => (
  <Link key={key} to={"/"} className={nv}>
    home
  </Link>
);

export const ToLearn = (key: number) => (
  <Link key={key} to={"/learn"} className={nv}>
    learn
  </Link>
);

export const ToCreate = (key: number) => (
  <Link key={key} to={"/create"} className={nv}>
    create
  </Link>
);

export const ToStudy = (key: number) => (
  <Link key={key} to={"/study"} className={nv}>
    study
  </Link>
);

export const ToLogin = (key: number) => (
  <Link key={key} to={"/login"} className={nv}>
    login
  </Link>
);

const Links = [ToHome, ToLearn, ToCreate, ToStudy, ToLogin];

export const NavLinks = () => {
  return <>{Links.map((fn, i) => fn(i))}</>;
};
