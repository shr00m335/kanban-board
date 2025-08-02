import { useAtom } from "jotai";
import React from "react";
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
}

const BoardListItem = ({
  boardListIndex,
  itemIndex,
  item,
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
    if (draggingItem !== null) {
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

  const handleMouseMove = (e: React.MouseEvent) => {
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
  };

  const ref = React.useRef<HTMLParagraphElement>(null);
  React.useEffect(() => {
    if (!ref.current || boardListIndex !== 0 || itemIndex !== 0) return;
    setFirstItemLocation([
      ref.current.getBoundingClientRect().left - 130,
      ref.current.getBoundingClientRect().top - 50,
    ]);
  }, [ref]);

  return (
    <>
      <p
        ref={ref}
        className={`bg-white w-[224px] px-2 py-1.5 mt-2 rounded-xl ${
          isDragging ? "absolute" : "static"
        }`}
        style={{
          left: mousePos[0],
          top: mousePos[1],
        }}
        onMouseDown={handleMouseDown}
        onMouseUp={handleItemMouseUp}
      >
        {item}
      </p>
      {isDragging && (
        <div
          className="w-screen h-screen absolute top-0 left-0"
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
        />
      )}
    </>
  );
};

export default BoardListItem;
