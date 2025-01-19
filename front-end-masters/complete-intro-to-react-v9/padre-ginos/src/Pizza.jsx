// export const Pizza = (props) => {
//   return React.createElement("div", {}, [
//     React.createElement("h1", {}, props.name),
//     React.createElement("p", {}, props.description),
//   ]);
// };

export const Pizza = (props) => {
  let p;
  return (
    <div className="">
      <h1>{props.name}</h1>
      <p>{props.description}</p>
    </div>
  );
};
