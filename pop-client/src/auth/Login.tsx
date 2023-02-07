import React, { useState } from "react";
import { Auth } from "../api/auth";
import "./styles.css";

const Login = () => {
  const [{ email, pw }, setFormState] = useState({
    email: "",
    pw: ""
  });

  const handleLogin = async (e: any) => {
    e.preventDefault();
    const res = await Auth.login(email, pw);
    console.log(res);
  };

  const updateFormState = (e: any) => {
    const { name, value } = e.target;
    setFormState((prev) => ({
      ...prev,
      [name]: value
    }));
  };

  return (
    <form className="login-form" onSubmit={handleLogin}>
      <input
        name="email"
        onChange={updateFormState}
        value={email}
        type="text"
        placeholder="EMAIL"
      />
      <input
        name="pw"
        onChange={updateFormState}
        value={pw}
        type="password"
        placeholder="PASSWORD"
      />
      <button type="submit">LOGIN</button>
    </form>
  );
};

export default Login;
