function LoadData({ columns, uuid, name }) {
  const columnsFormatted = columns.map((column) => `'${column}'`).join(", ");

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
    </div>
  );
}

export default LoadData;
