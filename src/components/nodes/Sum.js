import Node from "./Node";
import SourcePicker from "./SourcePicker";

function Sum({ name, uuid, nodes, source }) {
  return (
    <Node title="SUM" name={name}>
      <SourcePicker uuid={uuid} nodes={nodes} source={source} />
    </Node>
  );
}

export default Sum;
