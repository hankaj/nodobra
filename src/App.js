import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

import Board from "./components/Board";

function App() {
  let [nodes, setNodes] = useState({});
  let [result, setResult] = useState("");

  const addLoadData = async () => invoke("add_load_data");
  const addMultiply = async () => invoke("add_multiply");
  const addSum = async () => invoke("add_sum");

  const calculate = async (e) => {
    const uuid = document.getElementById("node-selection").value;
    console.log(`calculating ${uuid}`);
    invoke("calculate", { nodeId: uuid });
  };

  useEffect(() => {
    (async () => {
      const handles = [];

      handles.push(
        await listen("update_state", (event) => {
          console.log("got `update_state`");
          setNodes(event.payload.nodes);
        })
      );

      handles.push(
        await listen("show_result", (event) => {
          console.log("got `show_result`");
          console.log(event);
          setResult(event.payload.meta + "\n" + event.payload.result);
        })
      );

      handles.push(
        await listen("error", (event) => {
          console.log("got `error`");
          console.log(event);
          setResult(event.payload.message);
        })
      );

      invoke("get_nodes");

      return () => {
        handles.forEach((unlisten) => unlisten());
      };
    })();
  }, []);

  return (
    <div
      className="App"
      style={{
        padding: "1rem",
        display: "flex",
        flexDirection: "row",
        width: "100%",
        height: "100%",
      }}
    >
      <div style={{ flex: 0.8, height: "100%" }}>
        <Board nodes={nodes} />
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: 0.2,
          padding: "1rem",
        }}
      >
        <div style={{ display: "flex", flexDirection: "row" }}>
          <button onClick={addLoadData}>load csv</button>
          <button onClick={addMultiply}>multiply</button>
          <button onClick={addSum}>sum</button>
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
                {node.name}
              </option>
            ))}
          </select>
        </div>
        <pre>{result}</pre>
      </div>
    </div>
  );
}

export default App;
