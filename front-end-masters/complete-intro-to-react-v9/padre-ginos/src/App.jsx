import React, { StrictMode, useState } from "react";
import { createRoot } from "react-dom/client";
import Order from "./Order";
import PizzaOfTheDay from "./PizzaOfTheDay";
import Header from "./Header";
import { CartContext } from "./contexts";
// const HardPizza = () => {
//   // Array to return multiple top-level (sibling components)
//   // Same as stencil
//   return React.createElement("div", {}, [
//     React.createElement("h1", {}, "The Pepperoni Pizza"),
//     React.createElement("p", {}, "Mozzarella Cheese"),
//   ]);
// };

const App = () => {
  const cartHook = useState([]);
  return (
    <CartContext.Provider value={cartHook}>
      <div>
        <Header />
        <Order />
        <PizzaOfTheDay />
      </div>
    </CartContext.Provider>
  );
};

const container = document.getElementById("root");
const root = createRoot(container);
root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
