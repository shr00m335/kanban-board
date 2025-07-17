import React from "react";
import "./App.css";
import CreateProjectPopup from "./components/CreateProjectPopup";
import Sidebar from "./components/Sidebar";
import { Board } from "./components/board";

function App() {
  const [showPopup, setShowPopup] = React.useState<boolean>(false);

  return (
    <main className="w-screen h-screen grid grid-cols-[234px_auto]">
      <Sidebar onCreateClick={() => setShowPopup(true)} />
      {showPopup && (
        <CreateProjectPopup onCloseClick={() => setShowPopup(false)} />
      )}
      <Board />
    </main>
  );
}

export default App;
