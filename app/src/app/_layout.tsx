import {
	DarkTheme,
	DefaultTheme,
	ThemeProvider,
} from "@react-navigation/native";
import { Stack } from "expo-router";
import { StatusBar } from "expo-status-bar";
import { useColorScheme } from "react-native";
import { PortalHost } from "@rn-primitives/portal";

import "../global.css";
import { GoogleSignin } from "@react-native-google-signin/google-signin";

import { QueryClientProvider } from "@tanstack/react-query";
import { useReactQueryDevTools } from "@dev-plugins/react-query";

import { queryClient } from "@/lib/query-client";
import { useAuthStore } from "@/stores/auth";
import { useEffect } from "react";
import { getAccessToken } from "@/lib/auth-storage";
import { useCurrentUser } from "@/hooks/useCurrentUser";

GoogleSignin.configure({
	webClientId: process.env.EXPO_PUBLIC_GOOGLE_WEB_CLIENT_ID,
	iosClientId: process.env.EXPO_PUBLIC_GOOGLE_IOS_CLIENT_ID,
});

export default function RootLayout() {
	const colorScheme = useColorScheme();
	const setHasToken = useAuthStore((state) => state.setHasToken);

	useReactQueryDevTools(queryClient);

	useEffect(() => {
		getAccessToken().then((value) => {
			if (value) setHasToken();
		});
	}, []);

	return (
		<QueryClientProvider client={queryClient}>
			<ThemeProvider value={colorScheme === "dark" ? DarkTheme : DefaultTheme}>
				<StatusBar style={colorScheme === "dark" ? "light" : "dark"} />

				<Stack
					screenOptions={{
						headerBackButtonDisplayMode: "minimal",
					}}
				>
					<Stack.Screen name="(tabs)" options={{ headerShown: false }} />
					<Stack.Screen
						name="(auth)/welcome"
						options={{ title: "ようこそ！" }}
					/>
					<Stack.Screen
						name="profile"
						options={{
							title: "プロフィール",
							headerBackTitle: "マイページ",
						}}
					/>
					<Stack.Screen
						name="account/index"
						options={{
							title: "アカウント",
						}}
					/>
					<Stack.Screen
						name="account/email"
						options={{
							title: "メールアドレス",
						}}
					/>
				</Stack>

				<PortalHost />
			</ThemeProvider>
		</QueryClientProvider>
	);
}
