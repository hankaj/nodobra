function LoadData({ columns, uuid }) {
    return <pre>{uuid} load data {"{"} columns: {columns.join(", ")} {"}"}</pre>;
}

export default LoadData;