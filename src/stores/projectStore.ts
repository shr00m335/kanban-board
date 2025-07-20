import { atom } from "jotai";
import { BasicProjectInfo } from "../models/project";

export const allProjectsAtom = atom<BasicProjectInfo[]>([]);
