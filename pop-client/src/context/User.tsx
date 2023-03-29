import {
  createContext,
  useContext,
  useState,
  useCallback,
  useMemo
} from "react";

const UserContext = createContext({});

const useUserContext = () => {
  const context = useContext(UserContext);

  if (context === undefined) {
    throw new Error("useUserContext was used outside of its Provider");
  }

  return context;
};

const UserContextProvider = ({ children }: { children: any }) => {
  const [user, setUser] = useState<{} | undefined>({});

  const signout = useCallback(() => {
    setUser({});
  }, []);

  const signin = useCallback((newUser: any) => {
    if (!user) {
      setUser(newUser);
    } else {
      return user;
    }
  }, []);

  const value = {
    user,
    signout,
    signin
  };
  return <UserContext.Provider value={value}>{children}</UserContext.Provider>;
};

export { UserContextProvider, useUserContext };
