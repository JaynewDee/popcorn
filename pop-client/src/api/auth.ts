export const API = {
  login: async () =>
    await fetch("http://localhost:8000/api/login", {
      body: JSON.stringify({
        email: "jdiehl2236@gmail.com",
        pw: "supersecret"
      }),
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      }
    }).then((data) => data.json())
};
