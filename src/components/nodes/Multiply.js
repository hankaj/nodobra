import { invoke } from "@tauri-apps/api/tauri";

function Multiply({ name, uuid, nodes, source }) {
  const onSelect = (e) => {
    invoke("connect", { nodeUuid: uuid, sourceUuid: e.target.value });
  };

  const onUpdate = (e) => {
    const times = parseInt(e.target.value) || null;
    const patch = { uuid, kind: "multiply", data: { times } };
    invoke("update_node", { patch });
  };

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "fit-content",
        border: "1px solid black",
        padding: "0.5rem",
        margin: "0.5rem",
      }}
    >
      <pre>{`MULTIPLY\n---\nname: '${name}'`}</pre>
      <div style={{ display: "flex", flexDirection: "row" }}>
        <pre>source: </pre>
        <select onChange={onSelect} style={{ width: "fit-content" }}>
          <option value="" disabled selected hidden>
            select source
          </option>
          {Object.entries(nodes)
            .filter(([nodeUuid, _]) => nodeUuid !== uuid)
            .map(([uuid, node], i) => (
              <option value={uuid} key={i}>
                {node.data.name}
              </option>
            ))}
        </select>
      </div>
      <div style={{ display: "flex", flexDirection: "row" }}>
        <pre>times: </pre>
        <input type="text" onChange={onUpdate}></input>
      </div>
    </div>
  );
}

export default Multiply;
