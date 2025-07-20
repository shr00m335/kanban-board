import React from "react";
import { FaCheckCircle, FaTimesCircle } from "react-icons/fa";
import "./App.css";
import CreateProjectPopup from "./components/CreateProjectPopup";
import Sidebar from "./components/Sidebar";
import { Board } from "./components/board";

function App() {
  const [showPopup, setShowPopup] = React.useState<boolean>(false);
  const [showBanner, setShowBanner] = React.useState<boolean>(false);
  const [bannerSuccess, setBannerSuccess] = React.useState<boolean>(true);
  const [bannerMessage, setBannerMessage] = React.useState<string>("");

  const showMessage = (success: boolean, message: string): void => {
    setBannerSuccess(success);
    setBannerMessage(message);
    setShowBanner(true);
    setTimeout(() => {
      setShowBanner(false);
    }, 2500);
  };

  return (
    <main className="w-screen h-screen grid grid-cols-[234px_auto] overflow-y-hidden">
      <Sidebar onCreateClick={() => setShowPopup(true)} />
      {showPopup && (
        <CreateProjectPopup
          showBanner={showMessage}
          onCloseClick={() => setShowPopup(false)}
        />
      )}
      <Board />
      {/* Infomation Banner */}
      <div
        className={`w-min h-12 text-white absolute bottom-0 left-1/2 -translate-x-1/2 rounded-t-2xl flex items-center px-6 transition-transform duration-300 ${
          showBanner ? "translate-y-0" : "translate-y-12"
        } ${bannerSuccess ? "bg-green-700" : "bg-red-500"}`}
      >
        {bannerSuccess ? (
          <FaCheckCircle size={28} />
        ) : (
          <FaTimesCircle size={28} />
        )}
        <p className="text-lg ml-4 text-nowrap">{bannerMessage}</p>
      </div>
    </main>
  );
}

export default App;
