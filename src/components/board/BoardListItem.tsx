import { useAtom } from "jotai";
import React from "react";
import { FaTrash } from "react-icons/fa";
import { BoardListModel, BoardModel } from "../../models/project";
import {
  draggingItemAtom,
  draggingItemLocationAtom,
  firstItemLocationAtom,
} from "../../stores/dndStore";
import { openedBoardAtom } from "../../stores/projectStore";

interface BoardListItemProps {
  boardListIndex: number;
  itemIndex: number;
  item: string;
  showBanner: (success: boolean, message: string) => void;
}

const BoardListItem = ({
  boardListIndex,
  itemIndex,
  item,
  showBanner,
}: BoardListItemProps): JSX.Element => {
  const [firstItemLocation, setFirstItemLocation] = useAtom(
    firstItemLocationAtom
  );
  const [draggingItemLocation, setDraggingItemLocation] = useAtom(
    draggingItemLocationAtom
  );
  const [draggingItem, setDraggingItem] = useAtom(draggingItemAtom);

  const [isDragging, setIsDragging] = React.useState<boolean>(false);
  const [mousePos, setMousePos] = React.useState<number[]>([0, 0]);
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);
  const [isEditingItem, setIsEditingItem] = React.useState<boolean>(false);
  const [isDeleteOverlap, setIsDeleteOverlap] = React.useState<boolean>(false);

  const itemRef = React.useRef<HTMLParagraphElement>(null);
  const deleteAreaRef = React.useRef<HTMLDivElement>(null);

  let mouseHoldTimer: number | null = null;

  const handleMouseDown = (e: React.MouseEvent<HTMLParagraphElement>) => {
    e.stopPropagation();
    if (e.button !== 0) return;
    if (mouseHoldTimer !== null) clearTimeout(mouseHoldTimer);
    console.log(e);
    mouseHoldTimer = setTimeout(() => {
      const pos = [e.clientX - 112, e.clientY - 18];
      setMousePos(pos);
      const listLocation =
        pos[0] < firstItemLocation[0]
          ? -1
          : Math.floor((pos[0] - firstItemLocation[0]) / 270);
      const itemLocation =
        pos[1] < firstItemLocation[1]
          ? -1
          : Math.floor((pos[1] - firstItemLocation[1]) / 40);
      setDraggingItemLocation({
        listIndex: listLocation,
        itemIndex: itemLocation,
      });
      setDraggingItem(item);
      setIsDragging(true);
    }, 100);
  };

  const handleItemMouseUp = (e: React.MouseEvent) => {
    if (e.button !== 0) return;
    if (mouseHoldTimer !== null) clearTimeout(mouseHoldTimer);
  };

  const handleMouseUp = (e: React.MouseEvent) => {
    if (e.button !== 0) return;
    setIsDragging(false);
    if (!openedBoard) return;
    if (isDeleteOverlap) {
      const newBoard = {
        ...openedBoard,
        lists: openedBoard.lists.map((list, idx) =>
          idx === boardListIndex
            ? { ...list, items: list.items.filter((_, i) => i !== itemIndex) }
            : list
        ),
      };
      setOpenedBoard(newBoard);
      console.log("Deleted");
    } else if (draggingItem !== null) {
      let newBoard = {
        ...openedBoard,
        lists: openedBoard.lists.map((list, idx) =>
          idx === boardListIndex
            ? { ...list, items: list.items.filter((_, i) => i !== itemIndex) }
            : list
        ),
      };

      newBoard = {
        ...newBoard,
        lists: newBoard.lists.map((list, idx) =>
          idx === draggingItemLocation.listIndex
            ? {
                ...list,
                items: [
                  ...list.items.slice(0, draggingItemLocation.itemIndex),
                  draggingItem,
                  ...list.items.slice(draggingItemLocation.itemIndex),
                ],
              }
            : list
        ),
      };

      setOpenedBoard(newBoard);
    }
    setDraggingItem(null);
    setDraggingItemLocation({ listIndex: -1, itemIndex: -1 });
  };

  const checkDeleteOverlap = (x: number, y: number): void => {
    setIsDeleteOverlap(
      x >= deleteArea.x1 &&
        x <= deleteArea.x2 &&
        y >= deleteArea.y1 &&
        y <= deleteArea.y2
    );
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    const pos = [e.clientX - 112, e.clientY - 18];
    setMousePos(pos);
    if (pos[1] < window.innerHeight - 100) {
      const listLocation =
        pos[0] < firstItemLocation[0]
          ? -1
          : Math.floor((pos[0] - firstItemLocation[0]) / 270);
      const itemLocation =
        pos[1] < firstItemLocation[1]
          ? -1
          : Math.floor((pos[1] - firstItemLocation[1]) / 40);
      setDraggingItemLocation({
        listIndex: listLocation,
        itemIndex: itemLocation,
      });
      setIsDeleteOverlap(false);
    } else {
      checkDeleteOverlap(e.clientX, e.clientY);
      setDraggingItemLocation({
        listIndex: -1,
        itemIndex: -1,
      });
    }
  };

  const handleItemDbClick = (
    e: React.MouseEvent<HTMLParagraphElement>
  ): void => {
    if (e.button !== 0) return; // Only left click
    setIsEditingItem(true);
    setTimeout(() => {
      if (itemRef.current !== null) {
        itemRef.current.focus();
        const selection = window.getSelection();
        const range = document.createRange();
        range.selectNodeContents(itemRef.current);
        selection?.removeAllRanges();
        selection?.addRange(range);
        itemRef.current?.focus();
      }
    }, 0);
  };

  const handleItemKeyDown = (
    e: React.KeyboardEvent<HTMLParagraphElement>
  ): void => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      itemRef.current?.blur();
    }
  };

  const handleItemBlur = (): void => {
    if (itemRef.current === null || openedBoard === null) return;
    const newItem: string = itemRef.current.innerHTML
      .trim()
      .replace(/<div>/g, "\n")
      .replace(/<\/div>|<br>/g, "");
    console.log(newItem);
    if (newItem.length === 0) {
      showBanner(false, "Item cannot be empty");
      itemRef.current.innerHTML = item;
    } else {
      let updatedList: BoardListModel = {
        ...openedBoard.lists[boardListIndex],
      };
      updatedList.items[itemIndex] = newItem;
      let updatedBoard: BoardModel = {
        ...openedBoard,
      };
      updatedBoard.lists[boardListIndex] = updatedList;
      setOpenedBoard(updatedBoard);
    }
    setIsEditingItem(false);
  };

  React.useEffect(() => {
    if (!itemRef.current || boardListIndex !== 0 || itemIndex !== 0) return;
    setFirstItemLocation([
      itemRef.current.getBoundingClientRect().left - 130,
      itemRef.current.getBoundingClientRect().top - 50,
    ]);
  }, [itemRef]);

  let deleteArea: {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
  } = { x1: 0, y1: 0, x2: 0, y2: 0 };

  React.useEffect(() => {
    if (deleteAreaRef.current === null) return;
    const rect = deleteAreaRef.current.getBoundingClientRect();
    deleteArea = {
      x1: rect.left,
      y1: rect.top,
      x2: rect.right,
      y2: rect.bottom,
    };
  });

  return (
    <>
      {isDragging && (
        <div
          className="w-screen h-screen absolute top-0 left-0 z-10"
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
        >
          <div
            ref={deleteAreaRef}
            className="absolute flex justify-center items-center text-white bg-red-500 rounded-t-xl w-[300px] h-10 bottom-0 left-1/2 hover:-z-10"
            style={{
              opacity: isDeleteOverlap ? 1 : 0.5,
            }}
          >
            <FaTrash size={24} />
          </div>
        </div>
      )}
      <p
        ref={itemRef}
        className={`bg-white w-[224px] px-2 py-1.5 mt-2 rounded-xl whitespace-pre-line border-black border-[1px] ${
          isDragging ? "absolute" : "static"
        }`}
        style={{
          left: mousePos[0],
          top: mousePos[1],
        }}
        contentEditable={isEditingItem}
        onMouseDown={handleMouseDown}
        onMouseUp={handleItemMouseUp}
        onDoubleClick={handleItemDbClick}
        onKeyDown={handleItemKeyDown}
        onBlur={handleItemBlur}
      >
        {item}
      </p>
    </>
  );
};

export default BoardListItem;
