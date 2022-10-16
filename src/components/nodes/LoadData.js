import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

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
    <div
      style={{
        border: "1px solid black",
        width: "fit-content",
        padding: "0.5rem",
        margin: "0.5rem",
      }}
    >
      <pre>{`LOAD DATA\n---\nname: '${name}'\ncolumns: ${columnsFormatted}`}</pre>
      <pre>file: </pre>
      <button onClick={onClick}>load csv</button>
    </div>
  );
}

export default LoadData;
