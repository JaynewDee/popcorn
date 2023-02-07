import { createContext, useContext, useState, useCallback } from "react";

const UserContext = createContext({});

const useUserContext = () => {
  const context = useContext(UserContext);

  if (context === undefined) {
    throw new Error("useUserContext was used outside of its Provider");
  }

  return context;
};

const UserContextProvider = ({ children }: { children: any }) => {
  const [user, setUser] = useState({});

  const signout = useCallback(() => {
    setUser({});
  }, []);

  return <UserContext.Provider value={user}>{children}</UserContext.Provider>;
};

export { UserContextProvider, useUserContext };
