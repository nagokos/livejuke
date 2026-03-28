import * as SecureStore from "expo-secure-store";

const ACCESS_TOKEN_KEY = "access_token";
const REFRESH_TOKEN_KEY = "refresh_token";

export async function saveAccessToken(token: string) {
	await SecureStore.setItemAsync(ACCESS_TOKEN_KEY, token);
}

export async function getAccessToken() {
	return await SecureStore.getItemAsync(ACCESS_TOKEN_KEY);
}

export async function saveRefreshToken(token: string) {
	await SecureStore.setItemAsync(REFRESH_TOKEN_KEY, token);
}

export async function getRefreshToken() {
	return await SecureStore.getItemAsync(REFRESH_TOKEN_KEY);
}

export async function clearAccessToken() {
	await SecureStore.deleteItemAsync(ACCESS_TOKEN_KEY);
}

export async function clearRefreshToken() {
	await SecureStore.deleteItemAsync(REFRESH_TOKEN_KEY);
}
