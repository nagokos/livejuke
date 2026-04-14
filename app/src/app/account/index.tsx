import { Pressable, View } from "react-native";
import { Text } from "@/components/ui/text";
import { ChevronRight } from "lucide-react-native";
import { router } from "expo-router";

export default function Mypage() {
	return (
		<View className="flex-1 bg-white">
			<View className="mt-10 mx-5 gap-10">
				<Pressable
					onPress={() => router.push("/account/email")}
					className="flex-row items-center"
				>
					<View className="flex-1 ml-5">
						<Text className="text-base font-bold">メールアドレス</Text>
						<Text className="text-sm text-gray-500 mt-1">
							kosuda0428@gmail.com
						</Text>
					</View>
					<ChevronRight color="#aaa" />
				</Pressable>

				<Pressable className="flex-row items-center">
					<View className="flex-1 ml-5">
						<Text className="text-base font-bold text-rose-500">
							ログアウト
						</Text>
					</View>
				</Pressable>
			</View>
		</View>
	);
}
