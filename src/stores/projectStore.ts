import { atom } from "jotai";
import { BoardModel, ProjectModel } from "../models/project";

export const allProjectsAtom = atom<ProjectModel[]>([]);
export const openedProjectAtom = atom<ProjectModel | null>(null);
export const openedBoardAtom = atom<BoardModel | null>(null);
