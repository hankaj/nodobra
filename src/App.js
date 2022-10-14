import './App.css';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog';
import { useEffect, useState } from "react";

import LoadData from './components/nodes/LoadData';
import Multiply from './components/nodes/Multiply';
import Average from './components/nodes/Average';

function App() {
  let [nodes, setNodes] = useState({ nodes: [], uuids: [] });
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

  const calculate = async (e) => {
    const uuid = document.getElementById("node-selection").value;
    console.log(`calculating ${uuid}`);
    invoke('calculate', { uuid });
  }

  useEffect(() => {
    (async () => {
      await listen('show-nodes', (event) => {
        let uuids = event.payload.map((node) => node.uuid);
        setNodes({ nodes: event.payload, uuids });
      });

      await listen('show-result', (event) => {
        setResult(event.payload.meta + "\n" + event.payload.result);
      });
    })();
  })

  return (
    <div className="App">
      {
        nodes.nodes.map(({ type, data, uuid }, i) => {
          if (type === "load-data") {
            return <LoadData {...data} uuid={uuid} key={i}/>;
          } else if (type === "multiply") {
            return <Multiply {...data} uuid={uuid} uuids={nodes.uuids} key={i}/>;
          } else if (type === "average") {
            return <Average {...data} uuid={uuid} uuids={nodes.uuids} key={i}/>;
          }

          return null;
        })
      }
      <div style={{ display: "flex", flexDirection: "row" }}>
        <button onClick={addLoader}>add loader</button>
        <button onClick={addMultiplier}>add multiplier</button>
        <button onClick={addAverager}>add averager</button>
      </div>
      <div style={{ display: "flex", flexDirection: "row" }}>
        <button onClick={calculate}>calculate</button>
        <select id="node-selection">
          <option value="" disabled selected hidden>select node</option>
          {nodes.uuids.map((uuid, i) => <option value={uuid} key={i}>{uuid}</option>)}
        </select>
      </div>
      <pre>{result}</pre>
    </div>
  );
}

export default App;
