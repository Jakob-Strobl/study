export default async function getPastOrder(order) {
  // wait asynchrnously for 2 seconds
  // await new Promise((res) => setTimeout(res, 2000));
  const response = await fetch(`/api/past-order/${order}`);
  const data = await response.json();
  return data;
}
