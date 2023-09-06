import { Database } from "bun:sqlite";
import { Post, isPostBody, User, isUserBody } from "./types";

const db = new Database("d4dj.sqlite");

// Posts
export const getPosts = () => db.query("SELECT * FROM posts").all() as Post[];
export const getPost = (id: string) => db.query("SELECT * FROM posts WHERE id = $id").get({ $id: parseInt(id) }) as Post;

export const createPost = (body: unknown) => {
    if (!isPostBody(body)) return { success: false, error: "Invalid body" };

    db.query("INSERT INTO posts (title, body, user_id) VALUES ($title, $body, $user_id)").run({ $title: body.title, $body: body.body, $user_id: body.user_id });
    return { success: true };
}

// Users
export const getUsers = () => db.query("SELECT * FROM users").all() as User[];
export const getUser = (id: string) => db.query("SELECT id, username, created_at FROM users WHERE id = $id").get({ $id: parseInt(id) }) as User;

export const createUser = async (body: unknown) => {
    if (!isUserBody(body)) return { success: false, error: "Invalid body" };

    db.query("INSERT INTO users (username, password) VALUES ($username, $password)").run({ $username: body.username, $password: await Bun.password.hash(body.password) });

    return { success: true };
}