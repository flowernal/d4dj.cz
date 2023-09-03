<script lang="ts">
    import { onMount } from "svelte";
    import { posts } from "./store";

    onMount(async () => {
        fetch("https://d4dj.cz/api/posts")
            .then((response) => response.json())
            .then((data) => {
                posts.setPosts(data);
            })
            .catch((error) => {
                console.log(error);
                return [];
            });
    });
</script>

<main>
    <h1>Posts</h1>

    {#each $posts as post}
    <div class="card">
        <p>ID: {post.id} | Created At: {post.created_at}</p>
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
