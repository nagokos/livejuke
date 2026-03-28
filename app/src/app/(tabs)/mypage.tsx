import { Image, View } from "react-native";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { Link, Redirect } from "expo-router";
import { useAuthStore } from "@/stores/auth";

export default function Mypage() {
	const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
	if (!isLoggedIn()) {
		return <Redirect href="/welcome" />;
	}

	return (
		<View className="flex-1 bg-white">
			<View className="flex-1 items-center justify-center">
				<Image
					source={require("@/assets/images/livejuke-icon.png")}
					className="w-28 h-28 rounded-3xl mb-6"
				/>
				<Text className="text-5xl font-bold text-gray-900 mb-3">LiveJuke</Text>
				<Text className="text-lg text-gray-500 text-center leading-7">
					ライブの感動を、{"\n"}みんなで共有しよう
				</Text>
			</View>

			<View className="px-7 pb-12 gap-3">
				<Button size={"lg"} className="h-12 rounded-xl bg-[#534AB7]">
					<Text className="text-white text-base font-semibold">
						Googleで続ける
					</Text>
				</Button>

				<Link href="/register" asChild>
					<Button
						variant={"outline"}
						size={"lg"}
						className="h-12 rounded-xl border-gray-300"
					>
						<Text className="text-gray-900 text-base font-semibold">
							メールで登録
						</Text>
					</Button>
				</Link>

				<Text className="text-sm text-gray-400 text-center mt-1">
					アカウントをお持ちの方は{" "}
					<Link href="/login" className="text-[#534AB7]">
						ログイン
					</Link>
				</Text>
			</View>
		</View>
	);
}
