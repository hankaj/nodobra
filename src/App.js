import './App.css';
import { Container } from './Container.js'
import { DndProvider } from 'react-dnd'
import { HTML5Backend } from 'react-dnd-html5-backend'

function App() {
  return (
    <div className="App">
    <DndProvider backend={HTML5Backend}>
      <div style={{ overflow: 'hidden', clear: 'both' }}>
        <Container></Container>
      </div>
      </DndProvider>
    </div>
  );
}

export default App;
