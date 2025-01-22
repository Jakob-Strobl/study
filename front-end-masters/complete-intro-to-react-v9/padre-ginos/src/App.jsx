import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import Order from "./Order";
import PizzaOfTheDay from "./PizzaOfTheDay";
// const HardPizza = () => {
//   // Array to return multiple top-level (sibling components)
//   // Same as stencil
//   return React.createElement("div", {}, [
//     React.createElement("h1", {}, "The Pepperoni Pizza"),
//     React.createElement("p", {}, "Mozzarella Cheese"),
//   ]);
// };

const App = () => {
  return (
    <div>
      <h1 className="logo">Padre Gino's - Order Now</h1>
      <Order />
      <PizzaOfTheDay />
    </div>
  );
};

const container = document.getElementById("root");
const root = createRoot(container);
root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
