export interface ProjectModel {
  id: number[];
  name: string;
  description: string;
  boards: BoardModel[];
}

export interface BoardModel {
  name: string;
  lists: BoardListModel[];
}

export interface BoardListModel {
  title: string;
  color: number[];
  items: string[];
}
