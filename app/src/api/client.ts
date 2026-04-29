import {
	getAccessToken,
	getRefreshToken,
	saveAccessToken,
	saveRefreshToken,
} from "@/lib/auth-storage";
import { useAuthStore } from "@/stores/auth";
import { paths } from "@/types/schema";
import createClient, { Middleware } from "openapi-fetch";
import { router } from "expo-router";
import { queryClient } from "@/lib/query-client";
import { isPublicEndpoint } from "@/lib/auth-endpoints";

const API_URL = process.env.EXPO_PUBLIC_API_URL;

if (!API_URL) {
	throw new Error("EXPO_PUBLIC_API_URL is not set");
}

const client = createClient<paths>({
	baseUrl: API_URL,
});

let isRefreshing = false;
let refreshPromise: Promise<string | null> | null = null;

const requestBodies = new WeakMap<Request, string>();

const authMiddleware: Middleware = {
	async onRequest({ request }) {
		console.log(`[API Request] ${request.method} ${request.url}`);

		if (request.method !== "GET" && request.method !== "HEAD") {
			const clone = request.clone();
			const body = await clone.text();
			requestBodies.set(request, body);
		}

		const token = await getAccessToken();
		if (token && request.url.startsWith(API_URL)) {
			request.headers.set("Authorization", `Bearer ${token}`);
		}
		return request;
	},
	async onResponse({ request, response }) {
		if (response.status !== 401 || isPublicEndpoint(request)) {
			return response;
		}

		if (isRefreshing && refreshPromise) {
			const newToken = await refreshPromise;
			if (!newToken) return response;
			return executeRetry(request, newToken);
		}

		isRefreshing = true;
		refreshPromise = refreshAccessToken();

		try {
			const newToken = await refreshPromise;
			if (newToken) {
				return await executeRetry(request, newToken);
			}
			const logout = useAuthStore.getState().logout;
			await logout();

			queryClient.clear();

			router.replace("/mypage");

			return response;
		} finally {
			isRefreshing = false;
			refreshPromise = null;
		}
	},
};

async function refreshAccessToken(): Promise<string | null> {
	try {
		const refreshToken = await getRefreshToken();
		if (!refreshToken) return null;

		const { data, error } = await client.POST("/auth/refresh", {
			body: { refresh_token: refreshToken },
		});

		if (error || !data) return null;

		await saveAccessToken(data.access_token);
		await saveRefreshToken(data.refresh_token);

		return data.access_token;
	} catch {
		return null;
	}
}

async function executeRetry(
	originalRequest: Request,
	newToken: string,
): Promise<Response> {
	const headers = new Headers(originalRequest.headers);
	headers.set("Authorization", `Bearer ${newToken}`);

	const body = requestBodies.get(originalRequest);
	const retryRequest = new Request(originalRequest.url, {
		method: originalRequest.method,
		headers,
		body: body ?? undefined,
	});

	requestBodies.delete(originalRequest);

	return fetch(retryRequest);
}

client.use(authMiddleware);
export { client };
