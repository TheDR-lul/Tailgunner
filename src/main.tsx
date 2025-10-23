import React from "react";
import ReactDOM from "react-dom/client";
import "./i18n"; // Инициализация i18n
import { PatternsProvider } from "./hooks/usePatterns";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <PatternsProvider>
      <App />
    </PatternsProvider>
  </React.StrictMode>,
);
