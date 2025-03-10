import { render } from "@testing-library/react";
import { expect, test, vi } from "vitest";
import createFetchMock from "vitest-fetch-mock";
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";
import { Route } from "../routes/contact.lazy";

const queryClient = new QueryClient();
const fetchMocker = createFetchMock(vi);
fetchMocker.enableMocks();

test("can submit contact form", async () => {
  fetchMocker.mockResponse(JSON.stringify({ status: "ok" }));
  const screen = render(
    <QueryClientProvider client={queryClient}>
      {/* Grabs just component from our exported route for the page */}
      <Route.options.component></Route.options.component>
    </QueryClientProvider>,
  );

  const name = screen.getByPlaceholderText("Name");
  const email = screen.getByPlaceholderText("Email");
  const message = screen.getByPlaceholderText("Message");

  const testData = {
    name: "Brian",
    email: "brian@test.com",
    message: "Is this thing on?",
  };

  // Fill out the form
  name.value = testData.name;
  email.value = testData.email;
  message.value = testData.message;

  // Submit
  const btn = screen.getByRole("button");
  btn.click();

  // Tell to wait for it to appear (polls until timeout)
  const h3 = await screen.findByRole("heading", { level: 3 });

  expect(h3.innerText).toContain("Submitted");

  const requests = fetchMocker.requests();
  expect(requests.length).toBe(1);
  expect(requests[0].url).contains("/api/contact");
  expect(fetchMocker).toHaveBeenCalledWith("/api/contact", {
    body: JSON.stringify(testData),
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
  });
});
