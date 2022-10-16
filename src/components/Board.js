import LoadData from "./nodes/LoadData";
import Sum from "./nodes/Sum";
import Multiply from "./nodes/Multiply";

function Board({ nodes }) {
  return (
    <div
      style={{
        width: "100%",
        height: "100%",
        overflowY: "scroll",
        border: "1px solid black",
        padding: "1rem",
      }}
    >
      {Object.entries(nodes).map(([uuid, { kind, data }], i) => {
        if (kind === "load_data") {
          return <LoadData {...data} uuid={uuid} key={i} />;
        } else if (kind === "multiply") {
          return <Multiply {...data} uuid={uuid} nodes={nodes} key={i} />;
        } else if (kind === "sum") {
          return <Sum {...data} uuid={uuid} nodes={nodes} key={i} />;
        }

        return null;
      })}
    </div>
  );
}

export default Board;
