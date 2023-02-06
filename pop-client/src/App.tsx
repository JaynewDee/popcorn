import { useState } from "react";
import "./App.css";
import Login from "./auth/Login";
import Header from "./components/Header";

const App = () => {
  return (
    <div className="App">
      <Header />
      <Login />
    </div>
  );
};

export default App;
