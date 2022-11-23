import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Node from "./Node";
import SourcePicker from "./SourcePicker";
import Field from "./Field";

function Multiply({ name, uuid, nodes, source }) {
  let [times, setTimes] = useState(null);

  const sendUpdate = ({ times }) => {
    const settings = { kind: "multiply", data: { times } };

    invoke("update_node", { nodeId: uuid, settings });
  };

  const onUpdate = (e) => {
    let newTimes = parseInt(e.target.value);

    if (isNaN(newTimes)) {
      newTimes = 0;
    }

    setTimes(newTimes);
    sendUpdate({ times: newTimes });
  };

  return (
    <Node title="MULTIPLY" name={name}>
      <SourcePicker uuid={uuid} nodes={nodes} source={source} />
      <Field name="times">
        <input type="text" value={times} onChange={onUpdate}></input>
      </Field>
    </Node>
  );
}

export default Multiply;
