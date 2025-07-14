import React from "react";
import "./App.css";
import CreateProjectPopup from "./components/CreateProjectPopup";
import Sidebar from "./components/Sidebar";

function App() {
  const [showPopup, setShowPopup] = React.useState<boolean>(false);

  return (
    <main className="w-screen h-screen">
      <Sidebar onCreateClick={() => setShowPopup(true)} />
      {showPopup && (
        <CreateProjectPopup onCloseClick={() => setShowPopup(false)} />
      )}
    </main>
  );
}

export default App;
