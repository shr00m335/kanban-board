export interface BasicProjectInfo {
  id: number[];
  name: string;
  description: string;
}

export interface Project {
  id: number[];
  name: string;
  description: string;
  boards: Board[];
}

export interface Board {}
