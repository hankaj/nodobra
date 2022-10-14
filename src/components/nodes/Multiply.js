import { invoke } from '@tauri-apps/api/tauri';

function Multiply({ uuid, uuids, source }) {
    const onSelect = (e) => {
        invoke("connect", { nodeUuid: uuid, sourceUuid: e.target.value });
    }

    return (
        <div style={{display: "flex", flexDirection: "row"}}>
            <pre>{uuid} multiply: </pre>
            <select onChange={onSelect} value={source || ""}>
                <option value="" disabled selected hidden>select node</option>
                {uuids.filter((otherUuid) => otherUuid !== uuid).map((uuid, i) => <option value={uuid} key={i}>{uuid}</option>)}
            </select>
        </div>
    );
}

export default Multiply;