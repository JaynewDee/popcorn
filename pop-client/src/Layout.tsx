import { Routes } from "./components/Routes";
import Page from "./Page";
import Header from "./components/Header";

const Layout = () => {
  return (
    <div className="main-layout">
      <Header />
      <hr className="header-rule" />
      <Page>
        <Routes />
      </Page>
    </div>
  );
};

export default Layout;
