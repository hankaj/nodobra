import './App.css';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog';
import { useEffect } from "react";

function App() {
  const txt = document.getElementById("txt");

  const onClick = async () => {
    const selected = await open({
      multiple: false,
    });

    if (selected === null) {
      txt.innerHTML = "cancelled";
    } else {
      invoke('my_custom_command', { filePath: selected })
    }
  };

  useEffect(() => {
    (async () => {
      const unlisten = await listen('show-data', (event) => {
        txt.innerHTML = JSON.stringify(event.payload);
      });
    })();
  })

  return (
    <div className="App">
      <button onClick={onClick}>CLICK</button>
      <p id="txt"></p>
    </div>
  );
}

export default App;
