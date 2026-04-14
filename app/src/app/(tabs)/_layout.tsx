import { Tabs } from "expo-router";
import { UserRound } from "lucide-react-native";

export default function TabLayout() {
	return (
		<Tabs
			screenOptions={{
				tabBarActiveTintColor: "black",
			}}
		>
			<Tabs.Screen
				name="index"
				options={{
					headerShown: false,
					title: "ホーム",
				}}
			/>
			<Tabs.Screen
				name="search"
				options={{
					title: "検索",
				}}
			/>
			<Tabs.Screen
				name="mypage"
				options={{
					title: "マイページ",
					tabBarIcon: ({ color }) => <UserRound color={color} />,
				}}
			/>
		</Tabs>
	);
}
