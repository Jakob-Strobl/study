// export const Pizza = (props) => {
//   return React.createElement("div", {}, [
//     React.createElement("h1", {}, props.name),
//     React.createElement("p", {}, props.description),
//   ]);
// };

// Contrived example to prove a point - It also won't update the value rendered.
let counter = 0;
export const Pizza = (props) => {
  counter += counter;

  return (
    <div className="pizza" onClick={() => (counter += 1)}>
      <h1>
        {props.name}
        {counter}
      </h1>
      <p>{props.description}</p>
      <img src={props.image ?? "https://picsum.photos/200"} alt={props.name} />
    </div>
  );
};
