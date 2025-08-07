import { invoke } from "@tauri-apps/api/core";
import { useAtom, useSetAtom } from "jotai";
import React from "react";
import { IoArrowBack, IoSettingsSharp } from "react-icons/io5";
import { CommandResult } from "../models/commandResult";
import { BoardModel, ProjectModel } from "../models/project";
import {
  allProjectsAtom,
  openedBoardAtom,
  openedProjectAtom,
} from "../stores/projectStore";
import { ContextMenu, ContextMenuButton } from "./contextMenu";
import { DeletePopup } from "./DeletePopup";
import SettingsPopup from "./SettingsPopup";

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

  const [showContextMenu, setShowContextMenu] = React.useState<boolean>(false);
  const [contenxtMenuLocation, setContextMenuLocation] = React.useState<{
    x: number;
    y: number;
  }>({ x: 0, y: 0 });
  const [contextMenuItem, setContextMenuItem] = React.useState<number>(-1);
  const [isAddingItem, setIsAddingItem] = React.useState<boolean>(false);
  const [isShowingDeletePopup, setIsShowingDeletePopup] =
    React.useState<boolean>(false);
  const [isShowingSettings, setIsShowingSettings] =
    React.useState<boolean>(false);

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

  const handleContextMenu = (
    e: React.MouseEvent<HTMLSpanElement>,
    idx: number
  ): void => {
    e.preventDefault();
    e.stopPropagation();
    setContextMenuItem(idx);
    const rect = (e.target as HTMLSpanElement).getBoundingClientRect();
    setContextMenuLocation({
      x: rect.right,
      y: rect.top,
    });
    setShowContextMenu(true);
  };

  const handleContextMenuClose = (): void => {
    setContextMenuItem(-1);
    setShowContextMenu(false);
  };

  const handleContextMenuOpen = (): void => {
    if (openedProject === null) {
      openProject(projects[contextMenuItem].id);
    } else {
      openBoard(openedProject.boards[contextMenuItem].name);
    }
    handleContextMenuClose();
  };

  const handleContextMenuRename = (): void => {
    const items = Array.from(
      document.querySelectorAll("div#items-container > span")
    ) as HTMLButtonElement[];
    items[contextMenuItem].contentEditable = "true";
    items[contextMenuItem].focus();
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(items[contextMenuItem]);
    selection?.removeAllRanges();
    selection?.addRange(range);
    setShowContextMenu(false);
  };

  const handleContextMenuDuplicate = (): void => {
    if (openedProject !== null) {
      const targetBoard: BoardModel = {
        ...openedProject.boards[contextMenuItem],
      };
      let nameCount: number = 2;
      const boardNames: string[] = openedProject.boards.map((x) => x.name);
      let newName: string = `${targetBoard.name} ${nameCount}`;
      while (boardNames.includes(newName)) {
        nameCount += 1;
        newName = `${targetBoard.name} ${nameCount}`;
      }
      targetBoard.name = newName;
      setOpenedProject({
        ...openedProject,
        boards: [...openedProject.boards, targetBoard],
      });
      showBanner(
        true,
        `Duplicated "${openedProject.boards[contextMenuItem].name}" as "${newName}"`
      );
    }
    handleContextMenuClose();
  };

  const handleContextMenuDelete = (): void => {
    setShowContextMenu(false);
    setIsShowingDeletePopup(true);
  };

  const handleConfirmDelete = async (): Promise<void> => {
    setIsShowingDeletePopup(false);
    if (openedProject === null) {
      const projectName = projects[contextMenuItem].name;
      const result = await invoke<CommandResult<string>>("delete_project", {
        projectId: projects[contextMenuItem].id,
      });
      if (result.success) {
        showBanner(true, `Deleted project "${projectName}"`);
        setProjects(projects.filter((_, idx) => idx !== contextMenuItem));
      } else {
        showBanner(false, result.message ?? "No error message");
      }
    } else {
      const boardName: string = openedProject.boards[contextMenuItem].name;
      const updatedProject: ProjectModel = {
        ...openedProject,
        boards: openedProject.boards.filter(
          (_, idx) => idx !== contextMenuItem
        ),
      };
      const result = await invoke<CommandResult<ProjectModel>>("save_project", {
        project: updatedProject,
      });
      if (!result.success || result.data === null) {
        showBanner(false, result.message ?? "No error message");
        return;
      } else {
        setOpenedProject(result.data ?? updatedProject);
      }
      showBanner(true, `Deleted board "${boardName}"`);
    }
    setContextMenuItem(-1);
  };

  const onItemKeyDown = (e: React.KeyboardEvent<HTMLSpanElement>): void => {
    if (e.key === "Enter") {
      e.preventDefault();
      (e.target as HTMLSpanElement).blur();
    }
  };

  const renameProject = async (
    projectIndex: number,
    newName: string
  ): Promise<void> => {
    const projectId = projects[projectIndex].id;
    const result = await invoke<CommandResult<ProjectModel>>("read_project", {
      projectId,
    });
    if (!result.success || !result.data) {
      showBanner(false, result.message ?? "No error message");
      return;
    }
    let project = result.data;
    const oldName = project.name;
    project.name = newName;
    const saveResult = await invoke<CommandResult<ProjectModel>>(
      "save_project",
      {
        project,
      }
    );
    if (!saveResult.success) {
      showBanner(false, result.message ?? "No error message");
      return;
    }
    setProjects([
      ...projects.slice(0, projectIndex),
      project,
      ...projects.slice(projectIndex + 1),
    ]);
    showBanner(true, `Renamed ${oldName} to ${newName}`);
  };

  const onItemBlur = (e: React.FocusEvent<HTMLSpanElement>): void => {
    console.log("blur");
    console.log(contextMenuItem);
    if (contextMenuItem === -1) return;
    // Rename item
    const targetSpan = e.target as HTMLSpanElement;
    targetSpan.contentEditable = "false";
    const newName = targetSpan.innerText.trim();
    if (newName.length === 0) {
      showBanner(false, `New name cannot be empty`);
      targetSpan.innerText = projects[contextMenuItem].name;
    } else if (newName.length > 255) {
      showBanner(false, `New name cannot exceed 255 characters`);
      targetSpan.innerText = projects[contextMenuItem].name;
    } else if (openedProject === null) {
      renameProject(contextMenuItem, newName);
    } else {
      const board = openedProject.boards[contextMenuItem];
      const oldName = board.name;
      board.name = newName;
      let updatedProject: ProjectModel = { ...openedProject };
      updatedProject.boards[contextMenuItem] = board;
      setOpenedProject(updatedProject);
      showBanner(true, `Renamed ${oldName} to ${newName}`);
    }
    setContextMenuItem(-1);
  };

  const handleExitProject = (): void => {
    setOpenedProject(null);
    setOpenedBoard(null);
  };

  return (
    <>
      <div className="w-[234px] h-full bg-white grid grid-rows-[52px_auto_52px]">
        {/* Title */}
        <div className="flex items-center px-2">
          {openedProject !== null && (
            <IoArrowBack
              className="absolute cursor-pointer hover:text-black/60"
              size={28}
              onClick={handleExitProject}
            />
          )}
          <h1 className="font-bold text-2xl mx-auto my-auto select-none">
            {openedProject === null ? (
              <div>
                <span className="text-blue-600">Kanban</span> Board
              </div>
            ) : (
              <span>{openedProject!.name}</span>
            )}
          </h1>
        </div>
        {/* Items */}
        <div
          id="items-container"
          className="overflow-y-auto select-none flex flex-col"
        >
          {openedProject === null
            ? projects.map((project, idx) => (
                <span
                  key={project.id.join("")}
                  className=" w-full text-left px-3 py-1 text-lg cursor-pointer hover:bg-black/10"
                  onClick={() => openProject(project.id)}
                  onContextMenu={(e) => handleContextMenu(e, idx)}
                  onKeyDown={onItemKeyDown}
                  onBlur={onItemBlur}
                >
                  {project.name}
                </span>
              ))
            : openedProject!.boards.map((board, idx) => (
                <span
                  key={board.name}
                  className={`w-full text-left px-3 py-1 text-lg cursor-pointer ${
                    contextMenuItem === idx
                      ? "bg-black/10"
                      : "hover:bg-black/10"
                  }`}
                  onClick={() => openBoard(board.name)}
                  onContextMenu={(e) => handleContextMenu(e, idx)}
                  onKeyDown={onItemKeyDown}
                  onBlur={onItemBlur}
                >
                  {board.name}
                </span>
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
        <div className="flex">
          <button
            className="text-left px-3 my-auto text-gray-400 select-none hover:text-gray-600"
            onClick={onCreateBtnClick}
          >
            + Add {openedProject === null ? "Project" : "Board"}
          </button>
          <IoSettingsSharp
            className="ml-auto mr-3 my-auto cursor-pointer hover:text-black/5.0"
            size={24}
            onClick={() => setIsShowingSettings(true)}
          />
        </div>
      </div>
      {showContextMenu && (
        <ContextMenu
          x={contenxtMenuLocation.x}
          y={contenxtMenuLocation.y}
          onClose={handleContextMenuClose}
        >
          <ContextMenuButton onClick={handleContextMenuOpen}>
            Open
          </ContextMenuButton>
          <ContextMenuButton onClick={handleContextMenuRename}>
            Rename
          </ContextMenuButton>
          {openedProject !== null && (
            <ContextMenuButton onClick={handleContextMenuDuplicate}>
              Duplicate
            </ContextMenuButton>
          )}
          <ContextMenuButton onClick={handleContextMenuDelete}>
            <span className="text-red-500">Delete</span>
          </ContextMenuButton>
        </ContextMenu>
      )}
      {isShowingDeletePopup && (
        <DeletePopup
          deleteItem={
            openedProject === null
              ? projects[contextMenuItem].name
              : openedProject.boards[contextMenuItem].name
          }
          onClose={() => setIsShowingDeletePopup(false)}
          onConfirm={handleConfirmDelete}
        />
      )}
      {isShowingSettings && (
        <SettingsPopup
          showBanner={showBanner}
          onClose={() => setIsShowingSettings(false)}
        />
      )}
    </>
  );
};

export default Sidebar;
