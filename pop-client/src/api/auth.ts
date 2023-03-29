interface AuthEntity {
  username?: string;
  email: string;
  pw: string;
}

export const Auth = {
  login: async ({ email, pw }: AuthEntity) =>
    await fetch("/api/login", {
      body: JSON.stringify({
        email,
        pw
      }),
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      }
    })
      .then((data) => data.json())
      .catch((err) => console.error(err)),
  register: async ({ username, email, pw }: AuthEntity) =>
    await fetch("/api/register", {
      body: JSON.stringify({
        username,
        email,
        pw
      }),
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      }
    })
      .then((data) => data.json())
      .catch((err) => console.error(err))
};
