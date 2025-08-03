interface ContextMenuProps {
  x: number;
  y: number;
  children?: React.ReactNode;
  onClose: () => void;
}

export const ContextMenu = ({
  x,
  y,
  children,
  onClose,
}: ContextMenuProps): JSX.Element => {
  return (
    <div
      className="w-screen h-screen absolute left-0 top-0 px-2 "
      onClick={onClose}
    >
      <div
        className="absolute bg-white shadow-2xl min-w-[200px] flex flex-col rounded-lg"
        style={{ left: x, top: y }}
      >
        {children}
      </div>
    </div>
  );
};

interface ContextMenuButton {
  children: React.ReactNode;
  onClick?: () => void;
}

export const ContextMenuButton = ({
  children,
  onClick,
}: ContextMenuButton): JSX.Element => {
  return (
    <button
      className="text-nowrap px-2 py-1.5 text-left hover:bg-black/10 first:pt-2 first:pb-1.5 last:pt-1.5 last:pb-2"
      onClick={onClick}
    >
      {children}
    </button>
  );
};
