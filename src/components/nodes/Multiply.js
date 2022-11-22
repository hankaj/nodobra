import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Node from "./Node";
import SourcePicker from "./SourcePicker";
import Field from "./Field";

function Multiply({ name, uuid, nodes, source }) {
  let [times, setTimes] = useState(null);

  const onSelect = (e) => {
    invoke("add_edge", { destination: uuid, source: e.target.value });
  };

  const sendUpdate = ({ times }) => {
    const settings = { kind: "multiply", data: { times } };

    invoke("update_node", { nodeId: uuid, settings });
  };

  const onUpdate = (e) => {
    const newTimes = parseInt(e.target.value);
    setTimes(newTimes || null);
    sendUpdate({ times: newTimes });
  };

  return (
    <Node title="MULTIPLY" name={name}>
      <SourcePicker uuid={uuid} onSelect={onSelect} nodes={nodes} />
      <Field name="times">
        <input type="text" value={times} onChange={onUpdate}></input>
      </Field>
    </Node>
  );
}

export default Multiply;
