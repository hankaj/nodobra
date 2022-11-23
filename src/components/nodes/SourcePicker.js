import Field from "./Field";
import { invoke } from "@tauri-apps/api/tauri";

function SourcePicker({ nodes, uuid, source }) {
  const onSelect = (e) => {
    invoke("add_edge", { destination: uuid, source: e.target.value });
  };

  console.log(source);

  return (
    <Field name="source">
      <select
        onChange={onSelect}
        style={{ width: "fit-content" }}
        // value={source || ""}
      >
        <option value="" disabled selected hidden>
          select source
        </option>
        {Object.entries(nodes)
          .filter(([nodeUuid, _]) => nodeUuid !== uuid)
          .map(([uuid, node], i) => (
            <option value={uuid} key={i}>
              {node.name}
            </option>
          ))}
      </select>
    </Field>
  );
}

export default SourcePicker;
