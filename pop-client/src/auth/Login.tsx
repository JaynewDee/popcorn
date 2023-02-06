import React, { useState } from "react";
import { API } from "../api/auth";
import "./styles.css";
const Login = () => {
  const [formState, setFormState] = useState({
    username: "",
    pw: ""
  });

  const handleLogin = async (e: any) => {
    e.preventDefault();
    const res = await API.login();
    console.log(res);
  };
  return (
    <form className="login-form" onSubmit={handleLogin}>
      <label></label>
      <input type="text" placeholder="LOGIN"></input>
      <button type="submit">LOGIN</button>
    </form>
  );
};

export default Login;
