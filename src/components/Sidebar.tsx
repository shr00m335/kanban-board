import { invoke } from "@tauri-apps/api/core";
import { useAtom } from "jotai";
import React from "react";
import { CommandResult } from "../models/commandResult";
import { Project } from "../models/project";
import { allProjectsAtom } from "../stores/projectStore";

interface SidebarProp {
  showBanner: (success: boolean, message: string) => void;
  onCreateClick: () => void;
}

const Sidebar = ({
  showBanner,
  onCreateClick,
}: SidebarProp): React.ReactNode => {
  const [projects, setProjects] = useAtom(allProjectsAtom);

  React.useEffect(() => {
    invoke<CommandResult<Project[]>>("get_all_projects").then(
      (res: CommandResult<Project[]>) => {
        console.log(res);
        if (!res.success) {
          showBanner(false, res.message ?? "No error message");
          return;
        }
        setProjects(res.data ?? []);
      }
    );
  }, []);

  return (
    <div className="w-[234px] h-full bg-white grid grid-rows-[52px_auto_52px]">
      {/* Title */}
      <h1 className="font-bold text-2xl mx-auto my-auto select-none">
        <span className="text-blue-600">Kanban</span> Board
      </h1>
      {/* Items */}
      <div className="overflow-y-auto select-none">
        {projects.map((project) => (
          <p key={project.id.join("")} className="px-3 py-1 text-lg">
            {project.name}
          </p>
        ))}
      </div>
      <button
        className="text-left px-3 my-auto text-gray-400 select-none hover:text-gray-600"
        onClick={onCreateClick}
      >
        + Add Project
      </button>
    </div>
  );
};

export default Sidebar;
