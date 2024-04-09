import { createSignal } from "solid-js";

export default function Homepage() {
  const [count, setCount] = createSignal(0);

  return (
    <main>
      <h1 class="text-3xl font-bold underline">Hello world!</h1>
      <button class="increment" onClick={() => setCount(count() + 1)}>
        Clicks: {count()}
      </button>
      <p>
        Visit{" "}
        <a href="https://start.solidjs.com" target="_blank">
          start.solidjs.com
        </a>{" "}
        to learn how to build SolidStart apps.
      </p>
    </main>
  );
}