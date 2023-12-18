import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";

const root = document.getElementById("root") as HTMLDivElement;

const Main = () => (
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

createRoot(root).render(<Main />);
