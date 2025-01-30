const intl = new Intl.NumberFormat("en-us", {
  style: "currency",
  currency: "USD",
});

export const priceConverter = (price) => {
  return intl.format(price);
};
