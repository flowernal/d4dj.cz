export const actions = {
    post: async ({ request }: { request: Request }) => {
        const data = await request.formData();
        const title = data.get("title");
        const body = data.get("body");

        const response = await fetch("https://d4dj.cz/api/posts", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                title,
                body
            })
        });

        return await response.json();
    }
};
