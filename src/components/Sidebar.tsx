import React from "react";

const Sidebar = (): React.ReactNode => {
  return (
    <div className="w-[234px] h-full bg-white grid grid-rows-[52px_auto_52px]">
      {/* Title */}
      <h1 className="font-bold text-2xl mx-auto my-auto select-none">
        <span className="text-blue-600">Kanban</span> Board
      </h1>
      {/* Items */}
      <div className="overflow-y-auto select-none">
        <p className="px-3 py-1 text-lg">Item 1</p>
      </div>
      <button className="text-left px-3 my-auto text-gray-400 select-none hover:text-gray-600">
        + Add Project
      </button>
    </div>
  );
};

export default Sidebar;
