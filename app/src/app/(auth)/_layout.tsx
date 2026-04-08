import { useAuthStore } from "@/stores/auth";
import { Redirect, Stack } from "expo-router";

export default function AuthLayout() {
	const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
	if (isLoggedIn()) {
		return <Redirect href="/" />;
	}
	return <Stack screenOptions={{ headerShown: false }} />;
}
