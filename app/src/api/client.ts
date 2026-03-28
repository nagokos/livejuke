import { clearAccessToken, getAccessToken } from "@/lib/auth-storage";
import { useAuthStore } from "@/stores/auth";
import { paths } from "@/types/schema";
import createClient, { Middleware } from "openapi-fetch";
import { router } from "expo-router";

const API_URL = process.env.EXPO_PUBLIC_API_URL;

if (!API_URL) {
	throw new Error("EXPO_PUBLIC_API_URL is not set");
}

const client = createClient<paths>({
	baseUrl: API_URL,
});

const authMiddleware: Middleware = {
	async onRequest({ request }) {
		const token = await getAccessToken();

		if (token && request.url.startsWith(API_URL)) {
			request.headers.set("Authorization", `Bearer ${token}`);
		}
		return request;
	},
	async onResponse({ response }) {
		if (response.status === 401 && !response.url.includes("/auth/")) {
			await clearAccessToken();
			useAuthStore.getState().logout();
			router.replace("/welcome");
		}

		// request auth/refresh
		return response;
	},
};

client.use(authMiddleware);
export { client };
