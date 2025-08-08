import { invoke } from "@tauri-apps/api/core";
import { useAtom, useAtomValue } from "jotai";
import React from "react";
import { IoAddOutline } from "react-icons/io5";
import { CommandResult } from "../../models/commandResult";
import { BoardListModel, BoardModel, ProjectModel } from "../../models/project";
import {
  draggingListIndexAtom,
  draggingListLocationAtom,
} from "../../stores/dndStore";
import {
  configsAtom,
  openedBoardAtom,
  openedProjectAtom,
} from "../../stores/projectStore";
import { ContextMenu, ContextMenuButton } from "../ContextMenu";
import BoardList from "./BoardList";

interface BoardProps {
  showBanner: (success: boolean, message: string) => void;
}

const Board = ({ showBanner }: BoardProps): JSX.Element => {
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);
  const [openedProject, setOpenedProject] = useAtom(openedProjectAtom);
  const draggingListLocation = useAtomValue(draggingListLocationAtom);
  const draggingListIndex = useAtomValue(draggingListIndexAtom);
  const configs = useAtomValue(configsAtom);

  const [isAddingBoard, setIsAddingBoard] = React.useState<boolean>(false);
  const [lastSavedTime, setLastSavedTime] = React.useState<string | null>(null);
  const [isShowingContextMenu, setIsShowingContextMenu] =
    React.useState<boolean>(false);
  const [contextMenuPosition, setContextMenuPosition] = React.useState<{
    x: number;
    y: number;
  }>({ x: 0, y: 0 });

  const addBoardInputRef = React.useRef<HTMLInputElement>(null);
  const openedBoardRef = React.useRef(openedBoard);
  const openedProjectRef = React.useRef(openedProject);

  React.useEffect(() => {
    openedBoardRef.current = openedBoard;
  }, [openedBoard]);

  React.useEffect(() => {
    openedProjectRef.current = openedProject;
  }, [openedProject]);

  const onAddListClick = (): void => {
    setIsAddingBoard(true);
    setTimeout(() => {
      addBoardInputRef.current?.focus();
    });
  };

  const onAddListInputKeyDown = (
    e: React.KeyboardEvent<HTMLInputElement>
  ): void => {
    if (e.key === "Enter") {
      addBoardInputRef.current?.blur();
    }
  };

  const onAddListInputBlur = (): void => {
    if (openedBoard === null) return;
    let listName: string = addBoardInputRef.current?.value.trim() ?? "";
    if (listName.length === 0) {
      showBanner(false, "List title cannot be empty");
    } else if (openedBoard.lists.map((list) => list.title).includes(listName)) {
      showBanner(false, `${listName} already exists`);
    } else {
      const color = configs?.new_list_default_color ?? "#b6dfff";
      const newList: BoardListModel = {
        title: listName,
        color: [
          color.substring(1, 3),
          color.substring(3, 5),
          color.substring(5, 7),
        ].map((x) => Number(`0x${x}`)),
        items: [],
      };
      const updatedBoard: BoardModel = {
        ...openedBoard,
        lists: [...openedBoard.lists, newList],
      };
      setOpenedBoard(updatedBoard);
      console.log(updatedBoard);
    }
    if (addBoardInputRef.current) {
      addBoardInputRef.current.value = "";
    }
    setIsAddingBoard(false);
  };

  const listContainerRef = React.useRef<HTMLDivElement>(null);

  const saveProject = async (): Promise<boolean> => {
    if (openedProjectRef.current === null || openedBoardRef.current === null)
      return false;
    const currentBoardIndex = openedProjectRef.current.boards
      .map((x) => x.name)
      .indexOf(openedBoardRef.current.name);
    const updatedBoards = [
      ...openedProjectRef.current.boards.slice(0, currentBoardIndex),
      openedBoardRef.current,
      ...openedProjectRef.current.boards.slice(currentBoardIndex + 1),
    ];
    const updatedProject: ProjectModel = {
      ...openedProjectRef.current,
      boards: updatedBoards,
    };
    const result = await invoke<CommandResult<ProjectModel>>("save_project", {
      project: updatedProject,
    });
    if (!result.success || result.data === null) {
      showBanner(false, result.message ?? "No error message");
      return false;
    } else {
      setOpenedProject(result.data ?? updatedProject);
    }
    // Get time
    const now = new Date();
    setLastSavedTime(
      [now.getHours(), now.getMinutes(), now.getSeconds()]
        .map((x) => x.toString().padStart(2, "0"))
        .join(":")
    );
    console.log("Project Saved");
    return true;
  };

  let saveTimer: number | undefined;
  React.useEffect(() => {
    saveTimer = window.setInterval(async () => {
      await saveProject();
    }, (configs?.auto_save_interval ?? 60) * 1000);

    return () => {
      if (saveTimer !== undefined) {
        clearInterval(saveTimer);
      }
    };
  }, [configs]);

  const isLightColor = (color: string): boolean => {
    const [r, g, b] = [
      color.substring(1, 3),
      color.substring(3, 5),
      color.substring(5, 7),
    ].map((x) => Number(`0x${x}`));
    const luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    return luminance > 128;
  };

  const handleContextMenu = (e: React.MouseEvent<HTMLDivElement>): void => {
    e.preventDefault();
    setContextMenuPosition({ x: e.clientX, y: e.clientY });
    setIsShowingContextMenu(true);
  };

  const handleContextMenuSave = async (): Promise<void> => {
    if (await saveProject()) {
      showBanner(true, "Project Saved");
    }
    setIsShowingContextMenu(false);
  };

  const handleContextMenuClose = async (): Promise<void> => {
    await saveProject();
    setOpenedProject(null);
    setOpenedBoard(null);
  };

  return (
    <>
      <div
        className=" px-4 py-2.5 grid grid-rows-[52px_auto] select-none overflow-x-hidden"
        onContextMenu={handleContextMenu}
      >
        <div className="flex">
          <h1 className="text-2xl font-bold m-0">{openedBoard?.name ?? ""}</h1>
          <span className="ml-auto mt-0 text-sm text-gray-400">
            Last saved: {lastSavedTime ?? "Not saved"}
          </span>
        </div>
        <div
          className="flex w-full pt-1 pb-5 overflow-x-auto"
          ref={listContainerRef}
        >
          {openedBoard !== null &&
            openedBoard.lists.map((list, idx) => (
              <>
                {(draggingListIndex ?? -1) > idx &&
                  draggingListLocation === idx && (
                    <div className="min-w-[260px] h-full rounded-2xl px-4 py-2 ml-4 select-none first:ml-0"></div>
                  )}
                <BoardList
                  key={list.title}
                  boardList={list}
                  boardListIndex={idx}
                  showBanner={showBanner}
                  listContainerRef={listContainerRef}
                />
                {draggingListIndex !== null &&
                  draggingListIndex <= idx &&
                  draggingListLocation === idx && (
                    <div className="min-w-[260px] h-full rounded-2xl px-4 py-2 ml-4 select-none first:ml-0"></div>
                  )}
              </>
            ))}
          {draggingListIndex !== null &&
            draggingListLocation >= openedBoard!.lists.length && (
              <div className="min-w-[260px] h-full rounded-2xl px-4 py-2 ml-4 select-none first:ml-0"></div>
            )}
          <div
            className="grid-rows-[28px_auto_40px] w-[260px] h-full rounded-2xl px-4 py-2 ml-4 select-none first:ml-0"
            style={{
              display: isAddingBoard ? "grid" : "none",
              background: configs?.new_list_default_color ?? "#B6DfFF",
            }}
          >
            {/* Title */}
            <input
              ref={addBoardInputRef}
              className="text-lg font-bold my-auto"
              style={{
                color: isLightColor(
                  configs?.new_list_default_color ?? "#b6dfff"
                )
                  ? "black"
                  : "white",
              }}
              onKeyDown={onAddListInputKeyDown}
              onBlur={onAddListInputBlur}
            ></input>
            <div className="overflow-y-auto h-full"></div>
            {/* Add Button */}
            <button
              className={`text-left my-auto select-none ${
                isLightColor(configs?.new_list_default_color ?? "#b6dfff")
                  ? "text-gray-600 hover:text-gray-800"
                  : "text-gray-200 hover:text-gray-400"
              }`}
            ></button>
          </div>
          <button
            className="self-start min-w-10 min-h-10 bg-white flex ml-3 rounded-xl cursor-pointer hover:bg-white/50"
            onClick={onAddListClick}
          >
            <IoAddOutline className="mx-auto my-auto" size={32} />
          </button>
        </div>
      </div>
      {isShowingContextMenu && (
        <ContextMenu
          onClose={() => setIsShowingContextMenu(false)}
          x={contextMenuPosition.x}
          y={contextMenuPosition.y}
        >
          <ContextMenuButton onClick={handleContextMenuSave}>
            Save
          </ContextMenuButton>
          <ContextMenuButton onClick={handleContextMenuClose}>
            Close
          </ContextMenuButton>
        </ContextMenu>
      )}
    </>
  );
};

export default Board;
