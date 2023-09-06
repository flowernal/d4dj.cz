import { writable } from "svelte/store";
import type { Post, User } from "../../../backend/src/types";

interface UserPost extends Post {
    user: User
}

const initStore = () => {
    const initialPosts: UserPost[] = [];

    const { subscribe, set } = writable(initialPosts);

    return {
        subscribe,
        setPosts: (posts: UserPost[]) => set(posts),
        reset: () => set(initialPosts)
    };
};

export const posts = initStore();
