import { invoke } from "@tauri-apps/api/tauri";
import Node from "./Node";
import SourcePicker from "./SourcePicker";
import Field from "./Field";

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
    <Node title="MULTIPLY" name={name}>
      <SourcePicker uuid={uuid} onSelect={onSelect} nodes={nodes} />
      <Field name="times">
        <input type="text" onChange={onUpdate}></input>
      </Field>
    </Node>
  );
}

export default Multiply;
