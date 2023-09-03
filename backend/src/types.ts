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