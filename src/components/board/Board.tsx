import { IoAddOutline } from "react-icons/io5";
import BoardList from "./BoardList";

const Board = (): JSX.Element => {
  return (
    <div className="px-4 py-2.5 grid grid-rows-[52px_auto]">
      <h1 className="text-2xl font-bold">Board 1</h1>
      <div className="flex pb-5">
        <BoardList />
        <button className="self-start w-10 h-10 bg-white flex ml-3 rounded-xl cursor-pointer hover:bg-white/50">
          <IoAddOutline className="mx-auto my-auto" size={32} />
        </button>
      </div>
    </div>
  );
};

export default Board;
