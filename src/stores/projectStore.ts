import { atom } from "jotai";
import { Project } from "../models/project";

export const allProjectsAtom = atom<Project[]>([]);
export const openedProjectAtom = atom<Project | null>(null);
