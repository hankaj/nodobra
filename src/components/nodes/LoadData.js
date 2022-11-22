import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import Field from "./Field";
import Node from "./Node";

function LoadData({ columns, uuid, name }) {
  let [path, setPath] = useState(null);
  let [separator, setSeparator] = useState(",");

  const columnsFormatted = columns
    ? columns.map((column) => `'${column}'`).join(", ")
    : "";

  const sendUpdate = ({ path, separator }) => {
    const settings = { kind: "load_data", data: { path, separator } };
    console.log(settings);

    invoke("update_node", { nodeId: uuid, settings });
  };

  const onClick = async (e) => {
    const newPath = await open({
      multiple: false,
      defaultPath: path || undefined,
    });

    if (newPath != null) {
      setPath(newPath);
      sendUpdate({ path: newPath, separator });
    }
  };

  const onUpdate = (e) => {
    const newSeparator = e.target.value;
    setSeparator(newSeparator || null);
    sendUpdate({ path, separator: newSeparator });
  };

  return (
    <Node title="LOAD CSV" name={name}>
      <Field name="columns">{`${columnsFormatted}`}</Field>
      <Field name="file">
        <button onClick={onClick}>load csv</button>
      </Field>
      <Field name="separator">
        <input type="text" value={separator} onChange={onUpdate}></input>
      </Field>
    </Node>
  );
}

export default LoadData;
