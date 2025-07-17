import BoardList from "./BoardList";

const Board = (): JSX.Element => {
  return (
    <div className="px-4 py-2.5 grid grid-rows-[52px_auto]">
      <h1 className="text-2xl font-bold">Board 1</h1>
      <div className="flex pb-5">
        <BoardList />
      </div>
    </div>
  );
};

export default Board;
