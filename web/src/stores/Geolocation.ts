import { atom } from "nanostores";

export default {
    position: atom<number[]>(null),
    state: atom<string>()
}