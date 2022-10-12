function Average({ columns }) {
    return (
        <div style={{display: "flex", flexDirection: "row"}}>
            <pre>average: </pre>
            <select name="cars" id="cars">
                <option value="volvo">Volvo</option>
                <option value="saab">Saab</option>
                <option value="mercedes">Mercedes</option>
                <option value="audi">Audi</option>
            </select>
        </div>
    );
}

export default Average;