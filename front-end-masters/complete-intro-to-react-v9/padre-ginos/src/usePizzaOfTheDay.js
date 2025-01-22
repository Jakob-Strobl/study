import { useState, useEffect, useDebugValue } from "react";

export const usePizzaOfTheDay = () => {
  const [pizzaOfTheDay, setPizzaOfTheDay] = useState(null);
  useDebugValue(
    pizzaOfTheDay
      ? `${pizzaOfTheDay.id} : ${pizzaOfTheDay.name}`
      : "Loading...",
  );
  useEffect(() => {
    async function fetchPizzaOfTheDay() {
      const response = await fetch("/api/pizza-of-the-day");
      const data = await response.json();
      setPizzaOfTheDay(data);
    }

    fetchPizzaOfTheDay();
  }, []);

  // Seeing multiple still work. Has to be all called within the same render. :
  // useDebugValue("Returning!");
  return pizzaOfTheDay;
};
