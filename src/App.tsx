import { invoke } from "@tauri-apps/api/core";
import { useAtom, useAtomValue } from "jotai";
import React from "react";
import { FaCheckCircle, FaTimesCircle } from "react-icons/fa";
import "./App.css";
import { Board } from "./components/board";
import CreateProjectPopup from "./components/CreateProjectPopup";
import Sidebar from "./components/Sidebar";
import { CommandResult } from "./models/commandResult";
import { ConfigsModel } from "./models/configs";
import { BoardModel } from "./models/project";
import { configsAtom, openedBoardAtom } from "./stores/projectStore";

function App() {
  const openedBoard: BoardModel | null = useAtomValue(openedBoardAtom);
  const [configs, setConfigs] = useAtom(configsAtom);

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

  const getConfigs = async (): Promise<void> => {
    const result = await invoke<CommandResult<ConfigsModel>>("get_configs");
    if (!result.success || result.data === null) {
      showMessage(false, "Failed to get configs");
      setConfigs({
        auto_save_interval: 60,
        new_list_default_color: "#B6DFFF",
      });
    } else {
      setConfigs(result.data!);
    }
  };

  React.useEffect(() => {
    // Lock Scrolling
    const lockScroll = () => {
      window.scrollTo(0, 0);
    };
    window.addEventListener("scroll", lockScroll);
    // Read configs
    getConfigs();
    return () => {
      window.removeEventListener("scroll", lockScroll);
    };
  }, []);

  return (
    <main
      className="w-screen h-screen grid grid-cols-[234px_auto] overflow-hidden"
      onContextMenu={(e) => e.preventDefault()}
    >
      {configs !== null ? (
        <>
          <Sidebar
            showBanner={showMessage}
            onCreateClick={() => setShowPopup(true)}
          />
          {showPopup && (
            <CreateProjectPopup
              showBanner={showMessage}
              onCloseClick={() => setShowPopup(false)}
            />
          )}
          {openedBoard !== null && <Board showBanner={showMessage} />}

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
        </>
      ) : (
        <h1 className="absolute left-1/2 top-1/2">Loading Configs...</h1>
      )}
    </main>
  );
}

export default App;
