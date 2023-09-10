import { Elysia } from "elysia";
import { cookie } from "@elysiajs/cookie";
import { jwt } from "@elysiajs/jwt";
import { createAttachment, createPost, createUser, getAttachment, getAttachments, getAttachmentsByPostId, getPost, getPosts, getUser, getUsers, login } from "./routes";

const app = new Elysia()
    .use(
        jwt({
            name: "jwt",
            secret: Bun.env.JWT_SECRET || "secret",
        })
    )
    .use(cookie())

    .get("/api/posts", getPosts)
    .get("/api/posts/:id", ({ params: { id } }) => getPost(id))
    .post("/api/posts", ({ body }) => createPost(body))

    .get("/api/users", getUsers)
    .get("/api/users/:id", ({ params: { id } }) => getUser(id))
    .post("/api/users", ({ body }) => createUser(body))
    .post("/api/users/login/:id", async ({ params: { id }, body, jwt, cookie, setCookie, params }) => {
        if ((await login(id, body)).success) {
            setCookie("auth", await jwt.sign(params), {
                httpOnly: true,
                maxAge: 7 * 86400,
            })

            return { success: true, cookie: cookie.auth };
        };

        return { success: false, error: "Something went wrong." };
    })
    
    .get("/api/attachments", getAttachments)
    .get("/api/attachments/post/:id", ({ params: { id } }) => getAttachmentsByPostId(id))
    .get("/api/attachments/:id", ({ params: { id } }) => getAttachment(id))
    .post("/api/attachments", ({ body }) => createAttachment(body))

    .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);
