interface DeletePopupProps {
  deleteItem: string;
  onClose: () => void;
  onConfirm: () => void;
}

export const DeletePopup = ({
  deleteItem,
  onClose,
  onConfirm,
}: DeletePopupProps): JSX.Element => {
  return (
    <div className="absolute flex items-center justify-center top-0 left-0 w-screen h-screen bg-black/30 ">
      <div className="bg-[#EFEFEF] w-[500px] h-52 flex flex-col justify-between px-4 py-3 rounded-xl">
        <h2 className="font-bold text-2xl ">Delete List</h2>
        <p className="mx-auto text-lg">
          Are you sure you want to delete <strong>{deleteItem}</strong>
        </p>
        <div className="flex ml-auto">
          <button
            className="bg-white px-6 py-1 rounded-xl mr-4"
            onClick={onClose}
          >
            Cancel
          </button>
          <button
            className="bg-red-500 text-white px-6 py-1 rounded-xl"
            onClick={onConfirm}
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  );
};
