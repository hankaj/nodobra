import Field from "./Field";

function SourcePicker({ nodes, onSelect, uuid }) {
  return (
    <Field name="source">
      <select onChange={onSelect} style={{ width: "fit-content" }}>
        <option value="" disabled selected hidden>
          select source
        </option>
        {Object.entries(nodes)
          .filter(([nodeUuid, _]) => nodeUuid !== uuid)
          .map(([uuid, node], i) => (
            <option value={uuid} key={i}>
              {node.data.name}
            </option>
          ))}
      </select>
    </Field>
  );
}

export default SourcePicker;
