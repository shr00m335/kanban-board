import { invoke } from "@tauri-apps/api/core";
import { useAtom, useSetAtom } from "jotai";
import React from "react";
import { CommandResult } from "../models/commandResult";
import { BoardModel, ProjectModel } from "../models/project";
import {
  allProjectsAtom,
  openedBoardAtom,
  openedProjectAtom,
} from "../stores/projectStore";

interface SidebarProp {
  showBanner: (success: boolean, message: string) => void;
  onCreateClick: () => void;
}

const Sidebar = ({
  showBanner,
  onCreateClick,
}: SidebarProp): React.ReactNode => {
  const [projects, setProjects] = useAtom(allProjectsAtom);
  const [openedProject, setOpenedProject] = useAtom(openedProjectAtom);
  const setOpenedBoard = useSetAtom(openedBoardAtom);

  React.useEffect(() => {
    invoke<CommandResult<ProjectModel[]>>("get_all_projects").then(
      (res: CommandResult<ProjectModel[]>) => {
        console.log(res);
        if (!res.success) {
          showBanner(false, res.message ?? "No error message");
          return;
        }
        setProjects(res.data ?? []);
      }
    );
  }, []);

  const openProject = async (projectId: number[]): Promise<void> => {
    const result = await invoke<CommandResult<ProjectModel>>("read_project", {
      projectId,
    });
    if (!result.success) {
      showBanner(false, result.message ?? "No error message");
      return;
    }
    setOpenedProject(result.data!);
  };

  const openBoard = (boardName: string): void => {
    if (openedProject === null) {
      showBanner(false, "No opened project.");
      return;
    }
    let board: BoardModel | undefined = openedProject.boards.find(
      (x) => x.name === boardName
    );
    if (board === undefined) {
      showBanner(false, "Board not found.");
      return;
    }
    setOpenedBoard(board);
  };

  return (
    <div className="w-[234px] h-full bg-white grid grid-rows-[52px_auto_52px]">
      {/* Title */}
      <h1 className="font-bold text-2xl mx-auto my-auto select-none">
        {openedProject === null ? (
          <div>
            <span className="text-blue-600">Kanban</span> Board
          </div>
        ) : (
          <span>{openedProject!.name}</span>
        )}
      </h1>
      {/* Items */}
      <div className="overflow-y-auto select-none">
        {openedProject === null
          ? projects.map((project) => (
              <button
                key={project.id.join("")}
                className=" w-full text-left px-3 py-1 text-lg hover:bg-black/10"
                onClick={() => openProject(project.id)}
              >
                {project.name}
              </button>
            ))
          : openedProject!.boards.map((board) => (
              <button
                key={board.name}
                className=" w-full text-left px-3 py-1 text-lg hover:bg-black/10"
                onClick={() => openBoard(board.name)}
              >
                {board.name}
              </button>
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
