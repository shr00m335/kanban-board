interface BoardListItemProps {
  item: String;
}

const BoardListItem = ({ item }: BoardListItemProps): JSX.Element => {
  return (
    <div className="bg-white px-2 py-1.5 mt-2 rounded-xl">
      <p className="">{item}</p>
    </div>
  );
};

export default BoardListItem;
