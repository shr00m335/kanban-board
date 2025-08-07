import { invoke } from "@tauri-apps/api/core";
import { useAtom } from "jotai";
import { CommandResult } from "../models/commandResult";
import { ConfigsModel } from "../models/configs";
import { configsAtom } from "../stores/projectStore";

interface SettingsPopupProps {
  showBanner: (success: boolean, message: string) => void;
  onClose: () => void;
}

const SettingsPopup = ({
  showBanner,
  onClose,
}: SettingsPopupProps): JSX.Element => {
  const [configs, setConfigs] = useAtom(configsAtom);

  if (configs === null) return <></>;

  const saveConfigs = async (): Promise<void> => {
    const result = await invoke<CommandResult<string>>("save_configs", {
      configs,
    });
    if (!result.success) {
      showBanner(
        false,
        "Failed to save configs: " + (result.message ?? "No error message")
      );
    } else {
      showBanner(true, "Configs saved");
      onClose();
    }
  };

  return (
    <div className="flex absolute left-0 top-0 w-screen h-screen bg-black/50">
      <div className="w-9/12 h-9/12 bg-[#EFEFEF] m-auto px-6 py-4 rounded-2xl flex flex-col">
        <h1 className="text-2xl font-bold">Settings</h1>
        <div className="mt-4">
          <h2 className="text-xl font-bold">Project</h2>
          <div className="text-lg py-1 grid grid-cols-2 gap-2">
            <p>Auto Save Interval</p>
            <select
              value={configs.auto_save_interval}
              onChange={(e) => {
                const newConfigs: ConfigsModel = {
                  ...configs,
                  auto_save_interval: parseInt(e.target.value),
                };
                setConfigs(newConfigs);
              }}
            >
              <option value={30}>30 seconds</option>
              <option value={60}>1 minute</option>
              <option value={120}>2 minutes</option>
              <option value={300}>5 minutes</option>
              <option value={600}>10 minutes</option>
              <option value={900}>15 minutes</option>
              <option value={1800}>30 minutes</option>
              <option value={3600}>1 hour</option>
            </select>
          </div>
        </div>
        <div className="mt-4">
          <h2 className="text-xl font-bold">Board</h2>
          <div className="text-lg py-1 grid grid-cols-2">
            <p>New List Default Color</p>
            <input
              className="w-full h-8"
              type="color"
              value={configs.new_list_default_color}
              onChange={(e) => {
                const newConfigs: ConfigsModel = {
                  ...configs,
                  new_list_default_color: e.target.value,
                };
                setConfigs(newConfigs);
              }}
            />
          </div>
        </div>
        <div className="flex ml-auto mt-auto text-xl text-white">
          <button
            className="bg-blue-600 w-40 py-1 rounded-xl mr-4"
            onClick={onClose}
          >
            Cancel
          </button>
          <button
            className="bg-blue-600 w-40 py-1 rounded-xl mr-4"
            onClick={saveConfigs}
          >
            Save
          </button>
        </div>
      </div>
    </div>
  );
};

export default SettingsPopup;
