import { Elysia } from "elysia";
import { cors } from "@elysiajs/cors";
import { createPost, createUser, getPost, getPosts, getUser, getUsers, login } from "./routes";

const app = new Elysia()
    .use(cors())
    .get("/api/posts", getPosts)
    .get("/api/posts/:id", ({ params: { id } }) => getPost(id))
    .post("/api/posts", ({ body }) => createPost(body))

    .get("/api/users", getUsers)
    .get("/api/users/:id", ({ params: { id } }) => getUser(id))
    .post("/api/users", ({ body }) => createUser(body))
    .post("/api/users/login/:id", ({ params: { id }, body }) => login(id, body))
    .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);