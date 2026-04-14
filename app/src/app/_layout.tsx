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

GoogleSignin.configure({
	webClientId: process.env.EXPO_PUBLIC_GOOGLE_WEB_CLIENT_ID,
	iosClientId: process.env.EXPO_PUBLIC_GOOGLE_IOS_CLIENT_ID,
});

export default function RootLayout() {
	const colorScheme = useColorScheme();

	return (
		<ThemeProvider value={colorScheme === "dark" ? DarkTheme : DefaultTheme}>
			<StatusBar style={colorScheme === "dark" ? "light" : "dark"} />

			<Stack
				screenOptions={{
					headerBackButtonDisplayMode: "minimal",
				}}
			>
				<Stack.Screen name="(tabs)" options={{ headerShown: false }} />

				<Stack.Screen name="(auth)" options={{ headerShown: false }} />

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
	);
}
