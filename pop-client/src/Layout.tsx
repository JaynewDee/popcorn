import { Route, Switch } from "react-router-dom";
import Login from "./pages/Login";
import Header from "./components/Header";
import Home from "./pages/Home";
import Learn from "./pages/Learn";
import Create from "./pages/Create";
import Study from "./pages/Study";

const Layout = () => {
  return (
    <div>
      <Header />
      <Switch>
        <Route exact path="/" component={Home} />
        <Route exact path="/learn" component={Learn} />
        <Route exact path="/create" component={Create} />
        <Route exact path="/study" component={Study} />
        <Route exact path="/login" component={Login} />
      </Switch>
    </div>
  );
};

export default Layout;
