import "./App.css";
import { UserContextProvider } from "./context/User";
import Layout from "./Layout";

const App = () => {
  return (
    <UserContextProvider>
      <div className="App">
        <Layout />
      </div>
    </UserContextProvider>
  );
};

export default App;
