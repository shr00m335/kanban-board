import "./App.css";
import CreateProjectPopup from "./components/CreateProjectPopup";
import Sidebar from "./components/Sidebar";

function App() {
  return (
    <main className="w-screen h-screen">
      <Sidebar />
      <CreateProjectPopup />
    </main>
  );
}

export default App;
