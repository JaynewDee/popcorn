export const Auth = {
  login: async (email: String, pw: String) =>
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
      .catch((err) => console.error(err))
};
