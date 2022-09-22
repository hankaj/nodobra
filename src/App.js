import './App.css';
import { invoke } from '@tauri-apps/api/tauri';

function App() {
  const onClick = () => {
    console.log("ASDFASDF")
    invoke('my_custom_command', { invokeMessage: 'Hello!' })
  };

  return (
    <div className="App">
      <button onClick={onClick}>CLICK</button>
    </div>
  );
}

export default App;
