import { useState } from "react";
import "./App.css";
import Login from "./auth/Login";
import Header from "./components/Header";
import { UserContextProvider } from "./context/User";
import { BrowserRouter, Route } from "react-router-dom";
import Landing from "./components/Landing";

const App = () => {
  return (
    <UserContextProvider>
      <div className="App">
        <Header />
        <BrowserRouter>
          <Route path="/" element={<Landing />} />
          <Route path="/login" element={<Login />} />
        </BrowserRouter>
      </div>
    </UserContextProvider>
  );
};

export default App;
