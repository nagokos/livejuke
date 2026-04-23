import React, { useLayoutEffect } from "react";
import { useNavigation } from "expo-router";
import Welcome from "@/components/features/Welcome";
import Settings from "@/components/features/Settings";
import { useCurrentUser } from "@/hooks/useCurrentUser";

export default function MypageTab() {
	const navigation = useNavigation();
	const { currentUser } = useCurrentUser();

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
