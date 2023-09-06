<script lang="ts">
    import { onMount } from "svelte";
    import { posts } from "./store";
    import type { Post } from "../../../backend/src/types";

    // const URL = "https://d4dj.cz/api";
    const URL = "http://130.61.180.252:3000/api";

    onMount(async () => {
        const posts_req = await fetch(`${URL}/posts`);
        const posts_json = await posts_req.json();

        const with_users = await Promise.all(
            posts_json.map(async (post: Post) => {
                const user_req = await fetch(`${URL}/users/${post.user_id}`);
                const user_json = await user_req.json();

                return {
                    ...post,
                    user: user_json,
                };
            })
        );
        
        posts.setPosts(with_users);
    });
</script>

<main>
    <h1>Posts</h1>

    {#each $posts as post}
    <div class="card">
        <p>ID: {post.id} | Created At: {post.created_at}</p>
        <p>ID: {post.user.id} | Username: {post.user.username} | Created At: {post.user.created_at}</p>
        <h2>{post.title}</h2>
        <p>{post.body}</p>
    </div>
    {/each}
</main>

<style>
    .card {
        padding: 1em;
        border: 1px solid #ccc;
        border-radius: 0.5em;
        margin-bottom: 1em;
    }

    .card h2 {
        margin-bottom: 0.5em;
    }

    .card p {
        margin-bottom: 0.5em;
    }

    .card:hover {
        border-color: #646cff;
    }

    .card:hover .post h2 {
        color: #646cff;
    }

    .card:hover .post p {
        color: #646cff;
    }
</style> 
