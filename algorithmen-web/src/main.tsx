import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import Szene from "./szene.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
    <Szene />
  </StrictMode>,
);
