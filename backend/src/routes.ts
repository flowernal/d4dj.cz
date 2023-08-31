import { Database } from "bun:sqlite";
import { Post, PostBody } from "./types";

const db = new Database("d4dj.sqlite");

export const getPosts = () => db.query("SELECT * FROM posts").all() as Post[];
export const getPost = (id: string) => db.query("SELECT * FROM posts WHERE id = $id").get({ $id: parseInt(id) }) as Post;
export const createPost = (body: PostBody) => db.query("INSERT INTO posts (title, body) VALUES ($title, $body)").run({ $title: body.title, $body: body.body });