import { useAtom } from "jotai";
import { IoAddOutline } from "react-icons/io5";
import { openedBoardAtom } from "../../stores/projectStore";
import BoardList from "./BoardList";

const Board = (): JSX.Element => {
  const [openedBoard, setOpenedBoard] = useAtom(openedBoardAtom);

  return (
    <div className="px-4 py-2.5 grid grid-rows-[52px_auto] select-none">
      <h1 className="text-2xl font-bold">{openedBoard?.name ?? ""}</h1>
      <div className="flex pb-5">
        {openedBoard !== null &&
          openedBoard.lists.map((list, idx) => (
            <BoardList key={list.title} boardList={list} boardListIndex={idx} />
          ))}
        <button className="self-start w-10 h-10 bg-white flex ml-3 rounded-xl cursor-pointer hover:bg-white/50">
          <IoAddOutline className="mx-auto my-auto" size={32} />
        </button>
      </div>
    </div>
  );
};

export default Board;
