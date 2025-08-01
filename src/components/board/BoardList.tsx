import { useAtom, useAtomValue } from "jotai";
import React from "react";
import { BoardListModel, BoardModel } from "../../models/project";
import { draggingItemLocationAtom } from "../../stores/dndStore";
import { openedBoardAtom } from "../../stores/projectStore";
import BoardListItem from "./BoardListItem";

interface BoardListProps {
  boardList: BoardListModel;
  boardListIndex: number;
  showBanner: (success: boolean, message: string) => void;
}

const BoardList = ({
  boardList,
  boardListIndex,
  showBanner,
}: BoardListProps): JSX.Element => {
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);
  const draggingItemLocation = useAtomValue(draggingItemLocationAtom);

  const [isAddingItem, setIsAddingItem] = React.useState<boolean>(false);
  const addItemInputRef = React.useRef<HTMLInputElement>(null);

  const onAddItemClick = (): void => {
    console.log("clicked");
    setIsAddingItem(true);
    setTimeout(() => {
      addItemInputRef.current?.focus();
    }, 10);
  };

  const onAddItemInputKeyDown = (
    e: React.KeyboardEvent<HTMLInputElement>
  ): void => {
    if (e.key === "Enter") {
      addItemInputRef.current?.blur();
    }
  };

  const onAddItemInputBlur = (): void => {
    if (openedBoard === null) return;
    const item = addItemInputRef.current?.value.trim() ?? "";
    if (item.length === 0) {
      showBanner(false, "Item cannot be empty");
    } else {
      const updatedList: BoardListModel = {
        ...boardList,
        items: [...boardList.items, item],
      };
      const updatedBoard: BoardModel = {
        ...openedBoard,
        lists: [
          ...openedBoard.lists.slice(0, boardListIndex),
          updatedList,
          ...openedBoard.lists.slice(boardListIndex + 1),
        ],
      };
      setOpenedBoard(updatedBoard);
    }
    if (addItemInputRef.current) {
      addItemInputRef.current.value = "";
    }
    setIsAddingItem(false);
  };

  return (
    <div className="grid grid-rows-[28px_auto_40px] w-[260px] h-full bg-blue-300 rounded-2xl px-4 py-2 ml-4 select-none first:ml-0">
      {/* Title */}
      <h2 className="text-lg font-bold my-auto">{boardList.title}</h2>
      {/* Items */}
      <div className="overflow-y-auto h-full">
        {boardList.items.map((item, idx) => (
          <>
            {boardListIndex == draggingItemLocation.listIndex &&
              idx === draggingItemLocation.itemIndex && (
                <div
                  key="placeholder"
                  className="w-[224px] h-[36px] mt-2"
                ></div>
              )}
            <BoardListItem
              key={boardList.title + item}
              boardListIndex={boardListIndex}
              itemIndex={idx}
              item={item}
            />
          </>
        ))}
        <input
          ref={addItemInputRef}
          className={`bg-white w-[224px] px-2 py-1.5 mt-2 rounded-xl`}
          style={{ display: isAddingItem ? "block" : "none" }}
          onKeyDown={onAddItemInputKeyDown}
          onBlur={onAddItemInputBlur}
        />
      </div>
      {/* Add Button */}
      <button
        className="text-left my-auto text-gray-400 select-none hover:text-gray-600"
        onClick={onAddItemClick}
      >
        + Add Item
      </button>
    </div>
  );
};

export default BoardList;
