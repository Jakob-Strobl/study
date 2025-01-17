const App = () => {
  return React.createElement(
    "div",
    {},
    React.createElement("h1", {}, "Padre Ginos")
  )
}

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);
const appEl = React.createElement(App);
root.render(appEl);
