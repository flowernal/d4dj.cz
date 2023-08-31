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