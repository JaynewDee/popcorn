import React, { useState } from "react";
import { useLocation } from "react-router-dom";
import { Auth } from "../api/auth";

const Login = () => {
  const [{ email, pw }, setFormState] = useState({
    email: "",
    pw: ""
  });

  const location = useLocation();
  const handleLogin = async (e: any) => {
    e.preventDefault();
    try {
      const { cookie_id } = await Auth.login(email, pw);
      if (cookie_id) {
        sessionStorage.setItem("session_key", cookie_id);
        console.log(location);
      }
    } catch (err) {
      console.error(err);
    }
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
