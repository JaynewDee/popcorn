import React, { Component } from "react";

import { Route, Redirect } from "react-router-dom";
import { useUserContext } from "../context/User";

export const useProtected = (
  Component: any,
  path: string,
  redirect: string = "/login"
) => {
  const { user } = useUserContext() as any;

  return (
    <>{user ? <Component exact path={path} /> : <Redirect to={redirect} />}</>
  );
};
