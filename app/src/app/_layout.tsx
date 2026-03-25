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

export default function RootLayout() {
	const colorScheme = useColorScheme();

	return (
		<ThemeProvider value={colorScheme === "dark" ? DarkTheme : DefaultTheme}>
			<StatusBar style={colorScheme === "dark" ? "light" : "dark"} />
			<Stack />
			<PortalHost />
		</ThemeProvider>
	);
}
