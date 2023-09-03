import { Elysia } from "elysia";
import { cors } from "@elysiajs/cors";
import { createPost, getPost, getPosts } from "./routes";

const app = new Elysia()
    .get("/api/posts", getPosts)
    .get("/api/posts/:id", ({ params: { id } }) => getPost(id))
    .post("/api/posts", ({ body }) => createPost(body))
    .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);