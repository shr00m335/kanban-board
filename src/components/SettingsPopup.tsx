const SettingsPopup = (): JSX.Element => {
  return (
    <div className="flex absolute left-0 top-0 w-screen h-screen bg-black/50">
      <div className="w-9/12 h-9/12 bg-[#EFEFEF] m-auto px-6 py-4 rounded-2xl flex flex-col">
        <h1 className="text-2xl font-bold">Settings</h1>
        <div className="mt-4">
          <h2 className="text-xl font-bold">Project</h2>
          <div className="text-lg py-1 grid grid-cols-2 gap-2">
            <p>Auto Save Interval</p>
            <select>
              <option>30 seconds</option>
              <option>1 minute</option>
              <option>2 minutes</option>
              <option>5 minutes</option>
              <option>10 minutes</option>
              <option>15 minutes</option>
              <option>30 minutes</option>
              <option>1 hour</option>
              <option>Never</option>
            </select>
          </div>
        </div>
        <div className="mt-4">
          <h2 className="text-xl font-bold">Board</h2>
          <div className="text-lg py-1 grid grid-cols-2">
            <p>New List Default Color</p>
            <input type="color" />
          </div>
        </div>
        <div className="flex ml-auto mt-auto text-xl text-white">
          <button className="bg-blue-600 w-40 py-1 rounded-xl mr-4">
            Cancel
          </button>
          <button className="bg-blue-600 w-40 py-1 rounded-xl mr-4">
            Save
          </button>
        </div>
      </div>
    </div>
  );
};

export default SettingsPopup;
