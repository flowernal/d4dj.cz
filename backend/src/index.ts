import { Elysia } from "elysia";
import { cors } from "@elysiajs/cors";
import { createPost, getPost, getPosts } from "./routes";

const app = new Elysia()
    .use(cors())
    .get("/posts", getPosts)
    .get("/posts/:id", ({ params: { id } }) => getPost(id))
    .post("/posts", ({ body }) => createPost(body))
    .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);