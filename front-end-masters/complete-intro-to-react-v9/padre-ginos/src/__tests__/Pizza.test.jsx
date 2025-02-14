import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, test } from "vitest";
import { Pizza } from "../Pizza.jsx";

afterEach(cleanup);

test("alt text renders on pizza image element", async () => {
  const name = "My Favorite Pizza";
  const src = "https://picsum.photos/200";

  const screen = render(
    <Pizza name={name} description="Super cool pizza" image={src} />,
  );

  const img = screen.getByRole("img");
  expect(img.src).toBe(src);
  expect(img.alt).toBe(name);
});

test("pizza has default image if none is provided", async () => {
  const screen = render(
    <Pizza name="Something else" description="Some other kind of pizza" />,
  );

  // OMG SCREEN IS STATEFUL? NO WONDER I'VE HAD SO MANY PROBLEMS IN THE PAST
  const img = screen.getByRole("img");
  expect(img.src).not.toBe("");
});
