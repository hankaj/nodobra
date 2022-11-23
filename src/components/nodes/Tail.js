import Node from "./Node";
import SourcePicker from "./SourcePicker";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Field from "./Field";

function Tail({ name, uuid, nodes, source }) {
  let [rowCount, setRowCount] = useState(null);

  const sendUpdate = ({ rowCount }) => {
    const settings = { kind: "tail", data: { row_count: rowCount } };

    console.log("update with", settings);

    invoke("update_node", { nodeId: uuid, settings });
  };

  const onUpdate = (e) => {
    let newRowCount = parseInt(e.target.value);

    if (isNaN(newRowCount)) {
      newRowCount = 0;
    }

    setRowCount(newRowCount);
    sendUpdate({ rowCount: newRowCount });
  };

  return (
    <Node title="TAIL" name={name}>
      <SourcePicker uuid={uuid} nodes={nodes} source={source} />
      <Field name="row count (optional)">
        <input type="text" value={rowCount} onChange={onUpdate}></input>
      </Field>
    </Node>
  );
}

export default Tail;
