import { useAtom, useAtomValue } from "jotai";
import { BoardListModel } from "../../models/project";
import { draggingItemLocationAtom } from "../../stores/dndStore";
import { openedBoardAtom } from "../../stores/projectStore";
import BoardListItem from "./BoardListItem";

interface BoardListProps {
  boardList: BoardListModel;
  boardListIndex: number;
}

const BoardList = ({
  boardList,
  boardListIndex,
}: BoardListProps): JSX.Element => {
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);
  const draggingItemLocation = useAtomValue(draggingItemLocationAtom);

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
      </div>
      {/* Add Button */}
      <button className="text-left my-auto text-gray-400 select-none hover:text-gray-600">
        + Add Item
      </button>
    </div>
  );
};

export default BoardList;
