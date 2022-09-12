import logo from './logo.svg';
import './App.css';
import { useDrag } from 'react-dnd';
// import { ItemTypes } from './Constants'

function Node({ isDragging, text }) {
  const [{ opacity }, dragRef] = useDrag(
    () => ({
      // type: ItemTypes.NODE,
      item: { text },
      collect: (monitor) => ({
        opacity: monitor.isDragging() ? 0.5 : 1
      })
    }),
    []
  )
  return (
    <div ref={dragRef} style={{ opacity }}>
      {text}
    </div>
  )
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
