import { Pressable, View } from "react-native";
import { Text } from "@/components/ui/text";
import {
	ChevronRight,
	MessageCircleQuestionMark,
	UserRound,
} from "lucide-react-native";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { router } from "expo-router";
import { Image } from "expo-image";
import { useAuthStore } from "@/stores/auth";

export default function Settings() {
	const current_user = useAuthStore((state) => state.currentUser);

	return (
		<View className="flex-1 bg-white">
			<View className="mt-10 mx-5">
				<Pressable
					className="flex-row items-center border-b border-gray-100 pt-2 pb-8"
					onPress={() => router.push("/profile")}
				>
					<View className="w-10 items-center justify-center">
						<Avatar className="size-12" alt="user avatar">
							<AvatarImage
								source={{ uri: current_user?.avatar_url ?? undefined }}
							/>
							<AvatarFallback className="bg-transparent">
								<Image
									source={require("@/assets/images/user_default_avatar.png")}
									style={{ width: "100%", height: "100%" }}
								/>
							</AvatarFallback>
						</Avatar>
					</View>
					<View className="flex-1 flex-row items-center ml-5">
						<Text className="text-base font-bold flex-1">プロフィール</Text>
						<ChevronRight color="#aaa" />
					</View>
				</Pressable>
			</View>
			<View className="mt-10 mx-5 gap-10">
				<Pressable
					className="flex-row items-center "
					onPress={() => router.push("/account")}
				>
					<View className="w-10 items-center justify-center">
						<UserRound />
					</View>
					<View className="flex-1 flex-row items-center ml-5">
						<Text className="text-base font-bold flex-1">アカウント</Text>
						<ChevronRight color="#aaa" />
					</View>
				</Pressable>
				<View className="flex-row items-center ">
					<View className="w-10 items-center justify-center">
						<MessageCircleQuestionMark />
					</View>
					<View className="flex-1 flex-row items-center ml-5">
						<Text className="text-base font-bold flex-1">問い合わせ</Text>
						<ChevronRight color="#aaa" />
					</View>
				</View>
			</View>
		</View>
	);
}
