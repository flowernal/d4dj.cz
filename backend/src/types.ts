// Posts

export interface Post {
    id: number;
    title: string;
    body: string;
    created_at: Date;
}

export interface PostBody {
    title: string;
    body: string;
}

export const isPostBody = (arg: any): arg is PostBody => {
    return typeof arg.title === "string" && typeof arg.body === "string";
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
