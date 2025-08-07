import { atom } from "jotai";
import { ConfigsModel } from "../models/configs";
import { BoardModel, ProjectModel } from "../models/project";

export const allProjectsAtom = atom<ProjectModel[]>([]);
export const openedProjectAtom = atom<ProjectModel | null>(null);
export const openedBoardAtom = atom<BoardModel | null>(null);
export const configsAtom = atom<ConfigsModel | null>(null);
