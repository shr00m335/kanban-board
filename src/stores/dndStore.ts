import { atom } from "jotai";

export const draggingItemAtom = atom<string | null>(null);
export const draggingItemLocationAtom = atom<{
  listIndex: number;
  itemIndex: number;
}>({
  listIndex: -1,
  itemIndex: -1,
});
export const draggingListIndexAtom = atom<number | null>(null);
export const draggingListLocationAtom = atom<number>(0);

export const firstItemLocationAtom = atom<number[]>([0, 0]);
export const firstListLocationAtom = atom<number>(0);
