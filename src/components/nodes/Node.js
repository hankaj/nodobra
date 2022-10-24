function Node({ title, name, children }) {
  return (
    <div
      style={{
        border: "1px solid black",
        width: "fit-content",
        padding: "0.5rem",
        margin: "0.5rem",
      }}
    >
      <pre>
        {title} ({name})
      </pre>
      {children}
    </div>
  );
}

export default Node;
