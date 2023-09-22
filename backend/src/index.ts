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
    .get("/api/users/register", async ({ jwt, setCookie, body }) => {
        const { name, email, password } = body as { name: unknown, email: unknown, password: unknown };

        if (typeof name != "string" || typeof email != "string" || typeof password != "string") {
            return {
                success: false,
                error: "Invalid body"
            };
        }

        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
            return {
                success: false,
                error: "Invalid email"
            };
        }

        if (!/^[a-zA-Z0-9]{3,20}$/.test(name)) {
            return {
                success: false,
                error: "Invalid name"
            };
        }

        if (!/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,64}$/.test(password)) {
            return {
                success: false,
                error: "Invalid Password"
            };
        }

        const user = await createUser({
            email,
            name,
            passwordHash: Bun.password.hash(password),
        });

        const token = await jwt.sign({
            id: email
        });

        setCookie("jwt", token, {
            httpOnly: true,
            maxAge: 1000 * 60 * 60 * 24 * 365,
        });

        return {
            success: user.success,
            token: token,
        };
    })

    .get("/api/attachments", getAttachments)
    .get("/api/attachments/post/:id", ({ params: { id } }) => getAttachmentsByPostId(id))
    .get("/api/attachments/:id", ({ params: { id } }) => getAttachment(id))
    .post("/api/attachments", ({ body }) => createAttachment(body))

    .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);
