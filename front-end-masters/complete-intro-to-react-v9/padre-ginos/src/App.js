const Pizza = (props) => {
  return React.createElement("div", {}, [
    React.createElement("h1", {}, props.name),
    React.createElement("p", {}, props.description),
  ]);
}


const HardPizza = () => {
  // Array to return multiple top-level (sibling components)
  // Same as stencil
  return React.createElement("div", {}, [
    React.createElement("h1", {}, "The Pepperoni Pizza"),
    React.createElement("p", {}, "Mozzarella Cheese"),
  ])
}

const App = () => {
  return React.createElement(
    "div",
    {},
    [
      React.createElement("h1", {}, "Padre Ginos"),
      // {} passes properties to pizza, it's empty since there's no values to pass
      React.createElement(HardPizza, {}, []),
      React.createElement(HardPizza),
      React.createElement(Pizza, {
        name: 'The Reign Supreme',
        description: 'The Monarch of Supreme Pizza'
      }),
      React.createElement(Pizza, {
        name: 'The Grass Is Greener',
        description: 'Veggie Lovers Paradise'
      }),
    ]
  )
}

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);
const appEl = React.createElement(App);
root.render(appEl);
