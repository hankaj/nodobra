import './App.css';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog';
import { useEffect, useState } from "react";

import LoadData from './components/nodes/LoadData';
import Multiply from './components/nodes/Multiply';
import Average from './components/nodes/Average';

function App() {
  let [nodes, setNodes] = useState([]);
  let [result, setResult] = useState("");

  const addLoader = async () => {
    const selected = await open({
      multiple: false,
    });

    if (selected != null) {
      invoke('add_loader', { filePath: selected })
    }
  };

  const addMultiplier = async () => invoke('add_multiplier');
  const addAverager = async () => invoke('add_averager');

  useEffect(() => {
    (async () => {
      await listen('show-data', (event) => {
        console.log(event.payload)
        setNodes(event.payload.nodes);
        setResult(event.payload.result);
      });
    })();
  })

  return (
    <div className="App">
      {
        nodes.map(({ type, data }, i) => {
          if (type === "load-data") {
            return <LoadData {...data} />;
          } else if (type === "multiply") {
            return <Multiply {...data} />;
          } else if (type === "average") {
            return <Average {...data} />;
          }
        })
      }
      <div style={{ display: "flex", flexDirection: "row" }}>
        <button onClick={addLoader}>add loader</button>
        <button onClick={addMultiplier}>add multiplier</button>
        <button onClick={addAverager}>add averager</button>
      </div>
      <pre>{result}</pre>
    </div>
  );
}

export default App;
