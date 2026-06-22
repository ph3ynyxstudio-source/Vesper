import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { VesperionOverlay } from "./components/VesperionPet/VesperionOverlay";

const isVesperionWindow =
  new URLSearchParams(window.location.search).get("window") === "vesperion";

document.documentElement.classList.toggle(
  "vesperion-overlay-document",
  isVesperionWindow,
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {isVesperionWindow ? <VesperionOverlay /> : <App />}
  </React.StrictMode>,
);
