import { BoardListModel } from "../../models/project";
import BoardListItem from "./BoardListItem";

interface BoardListProps {
  boardList: BoardListModel;
}

const BoardList = ({ boardList }: BoardListProps): JSX.Element => {
  return (
    <div className="grid grid-rows-[28px_auto_40px] w-[260px] h-full bg-blue-300 rounded-2xl px-4 py-2">
      {/* Title */}
      <h2 className="text-lg font-bold my-auto">{boardList.title}</h2>
      {/* Items */}
      <div className="overflow-y-auto h-full">
        {boardList.items.map((item) => (
          <BoardListItem item={item} />
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
