import { writable } from "svelte/store";
import { type Post } from "../../../backend/src/types";

const initStore = () => {
    const initialPosts: Post[] = [];

    const { subscribe, set } = writable(initialPosts);

    return {
        subscribe,
        setPosts: (posts: Post[]) => set(posts),
        reset: () => set(initialPosts)
    };
};

export const posts = initStore();