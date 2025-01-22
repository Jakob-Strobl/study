import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Pizza } from "./Pizza";
import Order from "./Order";

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
      <h1>Padre Gino's - Order Now</h1>
      <Order />

      <Pizza
        name="The Reign Supreme"
        description="The Monarch of Supreme Pizza"
        image="public/pizzas/ital_supr.webp"
      />
      <Pizza
        name="The Grass Is Greener"
        description="Veggie Lovers Paradise"
        image="public/pizzas/green_garden.webp"
      />
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
