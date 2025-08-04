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
import { openedBoardAtom, openedProjectAtom } from "../../stores/projectStore";
import BoardList from "./BoardList";

interface BoardProps {
  showBanner: (success: boolean, message: string) => void;
}

const Board = ({ showBanner }: BoardProps): JSX.Element => {
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);
  const [openedProject, setOpenedProject] = useAtom(openedProjectAtom);
  const draggingListLocation = useAtomValue(draggingListLocationAtom);
  const draggingListIndex = useAtomValue(draggingListIndexAtom);

  const [isAddingBoard, setIsAddingBoard] = React.useState<boolean>(false);
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
      const newList: BoardListModel = {
        title: listName,
        color: [0xb6, 0xdf, 0xff],
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

  const saveProject = async (): Promise<void> => {
    if (openedProjectRef.current === null || openedBoardRef.current === null)
      return;
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
      return;
    } else {
      setOpenedProject(result.data ?? updatedProject);
    }
    console.log("Project Saved");
  };

  let saveTimer: number | undefined;
  React.useEffect(() => {
    saveTimer = window.setInterval(async () => {
      await saveProject();
    }, 10000);

    return () => {
      if (saveTimer !== undefined) {
        clearInterval(saveTimer);
      }
    };
  }, []);

  return (
    <div className=" px-4 py-2.5 grid grid-rows-[52px_auto] select-none overflow-x-hidden">
      <h1 className="text-2xl font-bold">{openedBoard?.name ?? ""}</h1>
      <div className="flex w-full pb-5 overflow-x-auto" ref={listContainerRef}>
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
        {
          <div
            className="grid-rows-[28px_auto_40px] w-[260px] h-full bg-[#b6dfff] rounded-2xl px-4 py-2 ml-4 select-none first:ml-0"
            style={{
              display: isAddingBoard ? "grid" : "none",
            }}
          >
            {/* Title */}
            <input
              ref={addBoardInputRef}
              className="text-lg font-bold my-auto"
              onKeyDown={onAddListInputKeyDown}
              onBlur={onAddListInputBlur}
            ></input>
            <div className="overflow-y-auto h-full"></div>
            {/* Add Button */}
            <button className="text-left my-auto text-gray-400 select-none hover:text-gray-600">
              + Add Item
            </button>
          </div>
        }
        <button
          className="self-start min-w-10 min-h-10 bg-white flex ml-3 rounded-xl cursor-pointer hover:bg-white/50"
          onClick={onAddListClick}
        >
          <IoAddOutline className="mx-auto my-auto" size={32} />
        </button>
      </div>
    </div>
  );
};

export default Board;
