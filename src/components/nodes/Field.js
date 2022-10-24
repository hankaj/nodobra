function Field({ name, children }) {
  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      <pre>{name}: </pre>
      {children}
    </div>
  );
}

export default Field;
