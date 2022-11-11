import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import Field from "./Field";
import Node from "./Node";

function LoadData({ columns, uuid, name }) {
  let [lastOpenDir, setLastOpenDir] = useState(null);

  const columnsFormatted = columns
    ? columns.map((column) => `'${column}'`).join(", ")
    : "";

  const onClick = async (e) => {
    const path = await open({
      multiple: false,
      defaultPath: lastOpenDir || undefined,
    });

    if (path != null) {
      setLastOpenDir(path);
      const patch = { uuid, kind: "load_data", data: { path } };

      invoke("update_node", { patch });
    }
  };

  return (
    <Node title="LOAD CSV" name={name}>
      <Field name="columns">{`${columnsFormatted}`}</Field>
      <Field name="file">
        <button onClick={onClick}>load csv</button>
      </Field>
    </Node>
  );
}

export default LoadData;
