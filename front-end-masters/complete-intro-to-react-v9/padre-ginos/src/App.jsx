import React from "react";
import { createRoot } from "react-dom/client";
import { Pizza } from "./Pizza";

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
      <Pizza
        name="The Reign Supreme"
        description="The Monarch of Supreme Pizza"
      />
      <Pizza name="The Grass Is Greener" description="Veggie Lovers Paradise" />
    </div>
  );
};

const container = document.getElementById("root");
const root = createRoot(container);
root.render(<App />);
