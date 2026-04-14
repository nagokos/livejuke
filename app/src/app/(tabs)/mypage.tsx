import React, { useLayoutEffect } from "react";
import { useNavigation } from "expo-router";
import { useAuthStore } from "@/stores/auth";
import Welcome from "@/components/features/Welcome";
import Settings from "@/components/features/Settings";

export default function MypageTab() {
	const navigation = useNavigation();
	const currentUser = useAuthStore((state) => state.currentUser);

	useLayoutEffect(() => {
		navigation.setOptions({
			headerShown: !!currentUser,
			title: "マイページ",
		});
	}, [navigation, currentUser]);

	if (!currentUser) {
		return <Welcome />;
	}

	return <Settings />;
}
