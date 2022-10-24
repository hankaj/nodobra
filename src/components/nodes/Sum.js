import { invoke } from "@tauri-apps/api/tauri";
import Node from "./Node";
import SourcePicker from "./SourcePicker";

function Sum({ name, uuid, nodes, source }) {
  const onSelect = (e) => {
    invoke("connect", { nodeUuid: uuid, sourceUuid: e.target.value });
  };

  return (
    <Node title="SUM" name={name}>
      <SourcePicker uuid={uuid} nodes={nodes} onSelect={onSelect} />
    </Node>
  );
}

export default Sum;
