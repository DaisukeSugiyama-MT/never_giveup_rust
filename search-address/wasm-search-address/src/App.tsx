import "./App.css";
import useHooks from "./hooks";

function App() {
  const { onChange, address, text } = useHooks();

  return (
    <div className="App">
      <div></div>
      <h1>Vite + React</h1>
      <div>
        <input type="text" onChange={onChange} value={text}></input>
      </div>
      <div>
        {address.map((item) => {
          const key = `${item.address1}_${item.address2}_${item.address3}`;
          return (
            <div key={key}>
              <div>{item.zipcode}</div>
              <div>
                {item.address1}
                {item.address2}
                {item.address3}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default App;
