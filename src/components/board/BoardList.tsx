import { useAtom, useAtomValue } from "jotai";
import React from "react";
import { FaTrash } from "react-icons/fa";
import { BoardListModel, BoardModel } from "../../models/project";
import {
  draggingItemLocationAtom,
  draggingListIndexAtom,
  draggingListLocationAtom,
  firstListLocationAtom,
} from "../../stores/dndStore";
import { openedBoardAtom } from "../../stores/projectStore";
import { ContextMenu, ContextMenuButton } from "../ContextMenu";
import { DeletePopup } from "../DeletePopup";
import BoardListItem from "./BoardListItem";

interface BoardListProps {
  boardList: BoardListModel;
  boardListIndex: number;
  showBanner: (success: boolean, message: string) => void;
  listContainerRef: React.RefObject<HTMLDivElement>;
}

const BoardList = ({
  boardList,
  boardListIndex,
  showBanner,
  listContainerRef,
}: BoardListProps): JSX.Element => {
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);
  const draggingItemLocation = useAtomValue(draggingItemLocationAtom);
  const [firstListLocation, setFirstListLocation] = useAtom(
    firstListLocationAtom
  );
  const [draggingListIndex, setDraggingListIndex] = useAtom(
    draggingListIndexAtom
  );
  const [draggingListLocation, setDraggingListLocation] = useAtom(
    draggingListLocationAtom
  );

  const [isAddingItem, setIsAddingItem] = React.useState<boolean>(false);
  const addItemInputRef = React.useRef<HTMLInputElement>(null);
  const [isDragging, setIsDragging] = React.useState<boolean>(false);
  const [mousePos, setMousePos] = React.useState<number[]>([-1, -1]);
  const [dragOffset, setDragOffset] = React.useState<number[]>([0, 0]);
  const [listHeight, setListHeight] = React.useState<number>(0);
  const [isEditingTitle, setIsEditingTitle] = React.useState<boolean>(false);
  const [isDeleteOverlap, setIsDeleteOverlap] = React.useState<boolean>(false);
  const [isShowingDeletePopup, setIsShowingDeletePopup] =
    React.useState<boolean>(false);
  const [isShowingContextMenu, setIsShowingContextMenu] =
    React.useState<boolean>(false);
  const [contenxtMenuLocation, setContextMenuLocation] = React.useState<{
    x: number;
    y: number;
  }>({ x: 0, y: 0 });

  const listRef = React.useRef<HTMLDivElement>(null);
  const contextMenuColorPickerRef = React.useRef<HTMLInputElement>(null);

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

  let listHoldTimer: number | null = null;

  const onListMouseDown = (e: React.MouseEvent<HTMLDivElement>): void => {
    if (e.button !== 0 || listRef.current === null) return;
    listHoldTimer = setTimeout(() => {
      const rect = listRef.current!.getBoundingClientRect();
      setDraggingListIndex(boardListIndex);
      setListHeight(rect.height);
      setDragOffset([e.clientX - rect.left, e.clientY - rect.top]);
      setMousePos([rect.left, rect.top]);
      setDraggingListLocation(
        e.clientX + (listContainerRef.current?.scrollLeft ?? 0) <
          firstListLocation
          ? -1
          : Math.floor(
              (e.clientX +
                +(listContainerRef.current?.scrollLeft ?? 0) -
                firstListLocation) /
                280
            )
      );
      setIsDragging(true);
    }, 100);
  };

  const onListMouseUp = (e: React.MouseEvent<HTMLDivElement>): void => {
    if (e.button !== 0) return;
    if (listHoldTimer !== null) {
      clearTimeout(listHoldTimer);
      listHoldTimer = null;
    }
  };

  const onDndMouseMove = (e: React.MouseEvent<HTMLDivElement>): void => {
    setMousePos([e.clientX - dragOffset[0], e.clientY - dragOffset[1]]);
    if (e.clientX > 235) {
      setDraggingListLocation(
        e.clientX + (listContainerRef.current?.scrollLeft ?? 0) <
          firstListLocation
          ? -2
          : Math.floor(
              (e.clientX +
                (listContainerRef.current?.scrollLeft ?? 0) -
                firstListLocation) /
                280
            )
      );
      setIsDeleteOverlap(false);
    } else {
      setDraggingListLocation(-1);
      setIsDeleteOverlap(true);
    }
  };

  const onDndMouseUp = (e: React.MouseEvent<HTMLDivElement>): void => {
    if (e.button !== 0) return;
    setIsDragging(false);
    if (openedBoard === null) return;
    // Set new list location
    if (isDeleteOverlap) {
      setIsDeleteOverlap(false);
      setIsShowingDeletePopup(true);
    } else if (draggingListIndex !== null && draggingListLocation > -1) {
      let updatedBoard: BoardModel = {
        ...openedBoard,
        lists: openedBoard.lists.filter((_, idx) => idx !== draggingListIndex),
      };
      updatedBoard.lists = [
        ...updatedBoard.lists.slice(0, draggingListLocation),
        openedBoard.lists[draggingListIndex],
        ...updatedBoard.lists.slice(draggingListLocation),
      ];
      console.log(updatedBoard);
      setOpenedBoard(updatedBoard);
      setDraggingListIndex(null);
    }
    setDraggingListLocation(-1);
  };

  React.useEffect(() => {
    if (boardListIndex === 0 && listRef.current !== null) {
      setFirstListLocation(listRef.current.getBoundingClientRect().left - 50);
    }
  }, [listRef]);

  const titleRef = React.useRef<HTMLHeadingElement>(null);

  const onTitleDbClick = (e: React.MouseEvent<HTMLHeadingElement>): void => {
    if (e.button !== 0) return;
    setIsEditingTitle(true);
    setTimeout(() => {
      if (titleRef.current !== null) {
        titleRef.current.focus();
        const selection = window.getSelection();
        const range = document.createRange();
        range.selectNodeContents(titleRef.current);
        selection?.removeAllRanges();
        selection?.addRange(range);
      }
    }, 10);
  };

  const onTitleKeyDown = (e: React.KeyboardEvent<HTMLHeadingElement>): void => {
    if (e.key === "Enter") {
      e.preventDefault();
      titleRef.current?.blur();
    }
  };

  const onTitleBlur = (): void => {
    if (titleRef.current === null || openedBoard === null) return;
    const newTitle = titleRef.current.innerHTML.trim();
    if (newTitle.length > 255) {
      showBanner(false, "Title cannot exceed 255 characters");
      titleRef.current.innerHTML = boardList.title;
    } else {
      const updatedList: BoardListModel = {
        ...boardList,
        title: newTitle,
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
    setIsEditingTitle(false);
  };

  const confirmDeleteList = () => {
    if (openedBoard === null) return;
    const updatedBoard: BoardModel = {
      ...openedBoard,
      lists: openedBoard.lists.filter((_, idx) => idx !== boardListIndex),
    };
    setOpenedBoard(updatedBoard);
    setIsShowingDeletePopup(false);
    setDraggingListIndex(null);
  };

  const handleContextMenu = (e: React.MouseEvent<HTMLSpanElement>): void => {
    e.preventDefault();
    e.stopPropagation();
    setContextMenuLocation({
      x: e.clientX,
      y: e.clientY,
    });
    setIsShowingContextMenu(true);
  };

  const handleContextMenuColor = (
    e: React.ChangeEvent<HTMLInputElement>
  ): void => {
    if (openedBoard === null) return;
    const hex: string = e.target.value;
    const updatedList: BoardListModel = {
      ...boardList,
      color: [
        hex.substring(1, 3),
        hex.substring(3, 5),
        hex.substring(5, 7),
      ].map((x) => Number(`0x${x}`)),
    };
    setOpenedBoard({
      ...openedBoard,
      lists: [
        ...openedBoard.lists.slice(0, boardListIndex),
        updatedList,
        ...openedBoard.lists.slice(boardListIndex + 1),
      ],
    });
    setIsShowingContextMenu(false);
  };

  const handleContextMenuDelete = () => {
    setIsShowingDeletePopup(true);
    setIsShowingContextMenu(false);
  };

  const isLightColor = ([r, g, b]: number[]): boolean => {
    const luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    return luminance > 128;
  };

  return (
    <>
      <div
        ref={listRef}
        style={{
          position: isDragging ? "absolute" : "static",
          left: mousePos[0],
          top: mousePos[1] - 20,
          height: isDragging ? listHeight : "100%",
          background: `rgb(${boardList.color.join(",")})`,
        }}
        className="grid grid-rows-[28px_auto_40px] min-w-[260px] rounded-2xl px-4 py-2 ml-4 select-none first:ml-0"
        onMouseDown={onListMouseDown}
        onMouseUp={onListMouseUp}
        onContextMenu={handleContextMenu}
      >
        {/* Title */}
        <h2
          ref={titleRef}
          className="text-lg font-bold my-auto max-w-[224px] line-clamp-1 truncate"
          style={{ color: isLightColor(boardList.color) ? "black" : "white" }}
          onDoubleClick={onTitleDbClick}
          onKeyDown={onTitleKeyDown}
          onBlur={onTitleBlur}
          contentEditable={isEditingTitle}
        >
          {boardList.title}
        </h2>
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
                showBanner={showBanner}
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
          className={`text-left my-auto select-none ${
            isLightColor(boardList.color)
              ? "text-gray-600 hover:text-gray-800"
              : "text-gray-200 hover:text-gray-400"
          }`}
          onClick={onAddItemClick}
        >
          + Add Item
        </button>
      </div>
      {/* Drag and drop overlay */}
      {isDragging && (
        <div
          className="absolute w-screen h-screen top-0 left-0 z-10"
          onMouseMove={onDndMouseMove}
          onMouseUp={onDndMouseUp}
        >
          <div
            className="absolute flex items-center justify-center w-[235px] h-screen left-0 top-0 bg-red-500 text-white"
            style={{ opacity: isDeleteOverlap ? 0.9 : 0.5 }}
          >
            <FaTrash size={48} />
          </div>
        </div>
      )}
      {isShowingDeletePopup && (
        <DeletePopup
          deleteItem={boardList.title}
          onClose={() => setIsShowingDeletePopup(false)}
          onConfirm={confirmDeleteList}
        />
      )}
      {isShowingContextMenu && (
        <ContextMenu
          x={contenxtMenuLocation.x}
          y={contenxtMenuLocation.y}
          onClose={() => setIsShowingContextMenu(false)}
        >
          <ContextMenuButton
            onClick={() => contextMenuColorPickerRef.current?.click()}
          >
            <div className="flex items-center">
              <div
                className="w-5 h-5 rounded-full"
                style={{
                  background: `rgb(${boardList.color.join(",")})`,
                }}
              ></div>
              <span className="ml-2">Color</span>
              <input
                ref={contextMenuColorPickerRef}
                type="color"
                className="cursor-pointer opacity-0"
                onClick={(e) => e.stopPropagation()}
                onChange={handleContextMenuColor}
              />
            </div>
          </ContextMenuButton>
          <ContextMenuButton onClick={handleContextMenuDelete}>
            <span className="text-red-500">Delete</span>
          </ContextMenuButton>
        </ContextMenu>
      )}
    </>
  );
};

export default BoardList;
