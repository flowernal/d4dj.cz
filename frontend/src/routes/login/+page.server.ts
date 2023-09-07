export const actions = {
    post: async ({ request }: { request: Request }) => {
        const data = await request.formData();
        const username = data.get("username");
        const password = data.get("password");

        const response = await fetch("https://d4dj.cz/api/login", { // login endpoint bude real
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username,
                password
            })
        });

        return await response.json();
    }
};
