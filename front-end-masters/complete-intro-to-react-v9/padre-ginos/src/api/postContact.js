export default async function postContact(name, email, message) {
  const response = await fetch("/api/contact", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name, email, message }),
  });

  if (!response.ok) {
    // Tan-stack query will catch this and set state appropriately
    throw new Error("Network response was not ok.");
  }

  // Okay to not await - chains the promise
  return response.json();
}
