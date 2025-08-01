import { useAtom } from "jotai";
import React from "react";
import { IoAddOutline } from "react-icons/io5";
import { BoardListModel, BoardModel } from "../../models/project";
import { openedBoardAtom } from "../../stores/projectStore";
import BoardList from "./BoardList";

interface BoardProps {
  showBanner: (success: boolean, message: string) => void;
}

const Board = ({ showBanner }: BoardProps): JSX.Element => {
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);

  const [isAddingBoard, setIsAddingBoard] = React.useState<boolean>(false);
  const addBoardInputRef = React.useRef<HTMLInputElement>(null);

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
        title: `List ${openedBoard.lists.length + 1}`,
        items: [],
      };
      const updatedBoard: BoardModel = {
        ...openedBoard,
        lists: [...openedBoard.lists, newList],
      };
      setOpenedBoard(updatedBoard);
    }
    if (addBoardInputRef.current) {
      addBoardInputRef.current.value = "";
    }
    setIsAddingBoard(false);
  };

  return (
    <div className="px-4 py-2.5 grid grid-rows-[52px_auto] select-none">
      <h1 className="text-2xl font-bold">{openedBoard?.name ?? ""}</h1>
      <div className="flex pb-5">
        {openedBoard !== null &&
          openedBoard.lists.map((list, idx) => (
            <BoardList
              key={list.title}
              boardList={list}
              boardListIndex={idx}
              showBanner={showBanner}
            />
          ))}
        {
          <div
            className="grid-rows-[28px_auto_40px] w-[260px] h-full bg-blue-300 rounded-2xl px-4 py-2 ml-4 select-none first:ml-0"
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
          className="self-start w-10 h-10 bg-white flex ml-3 rounded-xl cursor-pointer hover:bg-white/50"
          onClick={onAddListClick}
        >
          <IoAddOutline className="mx-auto my-auto" size={32} />
        </button>
      </div>
    </div>
  );
};

export default Board;
