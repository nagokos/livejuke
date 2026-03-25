import { Tabs } from "expo-router";

export default function TabLayout() {
	return (
		<Tabs
			screenOptions={{
				headerShown: false,
				tabBarActiveTintColor: "#534AB7",
			}}
		>
			<Tabs.Screen
				name="index"
				options={{
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
				}}
			/>
		</Tabs>
	);
}
