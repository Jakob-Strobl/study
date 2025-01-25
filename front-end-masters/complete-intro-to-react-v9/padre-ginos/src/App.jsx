import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { routeTree } from "./routeTree.gen";

// const HardPizza = () => {
//   // Array to return multiple top-level (sibling components)
//   // Same as stencil
//   return React.createElement("div", {}, [
//     React.createElement("h1", {}, "The Pepperoni Pizza"),
//     React.createElement("p", {}, "Mozzarella Cheese"),
//   ]);
// };
const router = createRouter({ routeTree });

const App = () => {
  return <RouterProvider router={router} />;
};

const container = document.getElementById("root");
const root = createRoot(container);
root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
