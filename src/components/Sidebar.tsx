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

  const [isAddingItem, setIsAddingItem] = React.useState<boolean>(false);

  const addItemRef = React.useRef<HTMLInputElement>(null);

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

  const onCreateBtnClick = () => {
    if (openedProject === null) {
      onCreateClick();
    } else {
      setIsAddingItem(true);
      setTimeout(() => {
        addItemRef.current?.focus();
      }, 10);
    }
  };

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

  const onAddItemInputKeyDown = (
    e: React.KeyboardEvent<HTMLInputElement>
  ): void => {
    if (e.key === "Enter") {
      addItemRef.current?.blur();
    }
  };

  const onAddItemInputBlur = async (
    e: React.FocusEvent<HTMLInputElement>
  ): Promise<void> => {
    if (openedProject === null) return;
    const boardName = e.target.value.trim();
    if (boardName.length === 0) {
      showBanner(false, "Board name cannot be empty");
    } else if (
      openedProject.boards.map((board) => board.name).includes(boardName)
    ) {
      showBanner(false, `\"${boardName}\" already exists`);
    } else {
      // Create board
      const newBoard: BoardModel = {
        name: boardName,
        lists: [],
      };
      const updatedProject: ProjectModel = {
        ...openedProject,
        boards: [...openedProject.boards, newBoard],
      };
      // Save updated project
      const result = await invoke<CommandResult<ProjectModel>>("save_project", {
        project: updatedProject,
      });
      if (!result.success || result.data === null) {
        showBanner(false, result.message ?? "No error message");
        return;
      } else {
        setOpenedProject(result.data ?? updatedProject);
      }
    }
    if (addItemRef.current) {
      addItemRef.current.value = "";
    }
    setIsAddingItem(false);
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
        <input
          ref={addItemRef}
          className={`w-full text-left px-3 py-1 text-lg ${
            isAddingItem ? "static" : "hidden"
          }`}
          onKeyDown={onAddItemInputKeyDown}
          onBlur={onAddItemInputBlur}
        />
      </div>
      <button
        className="text-left px-3 my-auto text-gray-400 select-none hover:text-gray-600"
        onClick={onCreateBtnClick}
      >
        + Add {openedProject === null ? "Project" : "Board"}
      </button>
    </div>
  );
};

export default Sidebar;
