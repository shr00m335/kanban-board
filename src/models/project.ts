export interface Project {
  id: number[];
  name: string;
  description: string;
  boards: Board[];
}

export interface Board {
  name: string;
  lists: BoardList[];
}

export interface BoardList {
  title: string;
  items: string[];
}
