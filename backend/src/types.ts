// Posts
export interface Post {
    id: number;
    title: string;
    body: string;
    user_id: number;
    created_at: Date;
}

export interface PostBody {
    title: string;
    body: string;
    user_id: number;
}

export const isPostBody = (arg: any): arg is PostBody => {
    return typeof arg.title === "string" && typeof arg.body === "string" && typeof arg.user_id === "number";
}

// Users
export interface User {
    id: number;
    username: string;
    password: string;
    created_at: Date;
}

export interface UserBody {
    username: string;
    password: string;
}

export const isUserBody = (arg: any): arg is UserBody => {
    return typeof arg.username === "string" && typeof arg.password === "string";
}
