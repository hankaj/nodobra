import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";

import LoadData from "./components/nodes/LoadData";
import Multiply from "./components/nodes/Multiply";
import Average from "./components/nodes/Average";
import Board from "./components/Board";

function App() {
  let [nodes, setNodes] = useState({});
  let [result, setResult] = useState("");
  let [lastOpenDir, setLastOpenDir] = useState(null);

  const addLoadData = async () => {
    const selected = await open({
      multiple: false,
      defaultPath: lastOpenDir || undefined,
    });

    if (selected != null) {
      setLastOpenDir(selected);
      invoke("add_loader", { filePath: selected });
    }
  };
  const addMultiply = async () => invoke("add_multiplier");
  const addAverage = async () => invoke("add_averager");

  const calculate = async (e) => {
    const uuid = document.getElementById("node-selection").value;
    console.log(`calculating ${uuid}`);
    invoke("calculate", { uuid });
  };

  useEffect(() => {
    (async () => {
      await listen("show_nodes", (event) => {
        console.log("got `show_nodes`");
        setNodes(event.payload);
      });

      await listen("show_result", (event) => {
        console.log("got `show_result`");
        setResult(event.payload.meta + "\n" + event.payload.result);
      });

      // Putting `invoke("get_nodes")` here causes infinite update loop.
    })();
  });

  return (
    <div
      className="App"
      style={{
        padding: "1rem",
        display: "flex",
        flexDirection: "column",
        width: "100%",
        height: "100%",
      }}
    >
      <Board nodes={nodes} />
      <pre>{result}</pre>
      <div style={{ display: "flex", flexDirection: "row" }}>
        <button onClick={addLoadData}>load data</button>
        <button onClick={addMultiply}>multiply</button>
        <button onClick={addAverage}>average</button>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifySelf: "flex-end",
        }}
      >
        <button onClick={calculate}>calculate</button>
        <select id="node-selection">
          <option value="" disabled selected hidden>
            select node
          </option>
          {Object.entries(nodes).map(([uuid, node], i) => (
            <option value={uuid} key={i}>
              {node.data.name}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}

export default App;
