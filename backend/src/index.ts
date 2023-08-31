import { Elysia } from "elysia";
import { createPost, getPost, getPosts } from "./routes";
import { PostBody } from "./types";

const app = new Elysia()
    .get("/posts", getPosts)
    .get("/posts/:id", ({ params: { id } }) => getPost(id))
    .post("/posts", ({ body }) => createPost(body as PostBody))
    .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);