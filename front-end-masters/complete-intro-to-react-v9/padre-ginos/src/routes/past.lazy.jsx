import { useQuery } from "@tanstack/react-query";
import { createLazyFileRoute } from "@tanstack/react-router";
import { useState } from "react";
import getPastOrders from "../api/getPastOrders";
import getPastOrder from "../api/getPastOrder";
import Modal from "../Modal";
import { priceConverter } from "../../useCurrency";

export const Route = createLazyFileRoute("/past")({
  component: PastOrderRoute,
});

function PastOrderRoute() {
  const [page, setPage] = useState(1);
  const [focusedOrder, setFocusedOrder] = useState(null);
  const { isLoading: isLoadingPastOrders, data: pastOrdersData } = useQuery({
    queryKey: ["past-orders", page],
    queryFn: () => getPastOrders(page),
    staleTime: 30000,
  });

  const { isLoading: isLoadingPastOrder, data: pastOrderData } = useQuery({
    queryKey: ["past-order", focusedOrder],
    queryFn: () => getPastOrder(focusedOrder),
    staleTime: 24 * 60 * 60 * 1000,
    enabled: !!focusedOrder,
  });

  if (isLoadingPastOrders) {
    return (
      <div className="past-orders">
        <h2>Loading</h2>
      </div>
    );
  }

  return (
    <div className="past-orders">
      <table>
        <thead>
          <tr>
            <td>ID</td>
            <td>Date</td>
            <td>Time</td>
          </tr>
        </thead>
        <tbody>
          {pastOrdersData.map((order) => {
            return (
              <tr key={order.order_id}>
                <td>
                  <button onClick={() => setFocusedOrder(order.order_id)}>
                    {order.order_id}
                  </button>
                </td>
                <td>{order.date}</td>
                <td>{order.time}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <div className="pages">
        <button disabled={page <= 1} onClick={() => setPage(page - 1)}>
          Previous
        </button>

        <button
          disabled={pastOrdersData.length < 10}
          onClick={() => setPage(page + 1)}
        >
          Next
        </button>
      </div>
      {focusedOrder ? (
        <Modal>
          <h2>Order #{focusedOrder}</h2>
          {!isLoadingPastOrder ? (
            <table>
              <thead>
                <tr>
                  <td>Image</td>
                  <td>Name</td>
                  <td>Size</td>
                  <td>Quantity</td>
                  <td>Price</td>
                  <td>Total</td>
                </tr>
              </thead>
              <tbody>
                {pastOrderData.orderItems.map((pizza) => (
                  <tr key={`${pizza.pizzaTypeId}_${pizza.size}`}>
                    <td>
                      <img src={pizza.image} alt={pizza.name} />
                    </td>
                    <td>{pizza.price}</td>
                    <td>{pizza.size}</td>
                    <td>{pizza.quantity}</td>
                    <td>{priceConverter(pizza.price)}</td>
                    <td>{priceConverter(pizza.total)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          ) : (
            <p>Loading</p>
          )}
          <button onClick={() => setFocusedOrder(undefined)}>Close</button>
        </Modal>
      ) : null}
    </div>
  );
}
