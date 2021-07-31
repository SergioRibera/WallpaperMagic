import './App.css';
import ScreensView from './screens/screensview.jsx';
import Screen from './screens/screen.jsx';

import Dropdown from "./dropdown/dropdown.jsx";

function App() {
    const items = [
        "Mix", "Other", "Test"
    ];
  return (
    <div className="App">
        <ScreensView />
        <div className="screens-settings">
            <Screen sizeX={180} sizeY={19.2} />
            <div className="screens-settings-options">
                <Dropdown options={items} text="Resolution" defaultValue={items[0]}/>
            </div>
        </div>
    </div>
  );
}

export default App;
