import { Pressable, TouchableOpacity, View } from "react-native";
import { Text } from "@/components/ui/text";
import { ChevronRight } from "lucide-react-native";
import { router } from "expo-router";
import {
	Dialog,
	DialogClose,
	DialogContent,
	DialogDescription,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { client } from "@/api/client";
import { useAuthStore } from "@/stores/auth";
import { getRefreshToken } from "@/lib/auth-storage";
import { useCurrentUser } from "@/hooks/useCurrentUser";
import { queryClient } from "@/lib/query-client";

export default function Account() {
	const { currentUser } = useCurrentUser();

	const zustandLogout = useAuthStore((state) => state.logout);

	const logout = async () => {
		const token = await getRefreshToken();
		if (!token) {
			router.replace("/(tabs)");
			return;
		}

		try {
			await client.POST("/auth/logout", {
				body: {
					refresh_token: token,
				},
			});
		} catch (e) {
			console.log(e);
		} finally {
			console.log("logout");
			await zustandLogout();
			router.replace("/(tabs)");
		}
	};

	return (
		<View className="flex-1 bg-white">
			<View className="mt-10 mx-5 gap-7">
				<Pressable
					onPress={() => router.push("/account/email")}
					className="flex-row items-center"
				>
					<View className="flex-1 ml-5">
						<Text className="text-base font-bold">メールアドレス</Text>
						<Text className="text-sm text-gray-500 mt-1">
							{currentUser?.email}
						</Text>
					</View>
					<ChevronRight color="#aaa" />
				</Pressable>

				<Dialog>
					<DialogTrigger asChild>
						<TouchableOpacity className="flex-row items-center py-4">
							<View className="flex-1 ml-5">
								<Text className="text-base font-bold text-rose-500">
									ログアウト
								</Text>
							</View>
						</TouchableOpacity>
					</DialogTrigger>

					<DialogContent
						showClose={false}
						className="w-[270px] p-0 gap-0 rounded-[14px] bg-white/95 overflow-hidden border-0"
					>
						<View className="pt-5 pb-4 px-4 items-center">
							<DialogTitle className="text-center text-[17px] font-semibold text-black mb-1">
								ログアウトします
							</DialogTitle>
							<DialogDescription className="text-center text-[13px] text-gray-800">
								よろしいですか？
							</DialogDescription>
						</View>

						<View className="flex-row border-t border-gray-300/80 h-[44px]">
							<DialogClose asChild>
								<TouchableOpacity className="flex-1 justify-center items-center border-r border-gray-300/80">
									<Text className="text-gray-600 text-[17px]">キャンセル</Text>
								</TouchableOpacity>
							</DialogClose>

							<DialogClose asChild>
								<TouchableOpacity
									onPress={logout}
									className="flex-1 justify-center items-center"
								>
									<Text className="text-rose-500 text-[17px] font-semibold">
										ログアウト
									</Text>
								</TouchableOpacity>
							</DialogClose>
						</View>
					</DialogContent>
				</Dialog>
			</View>
		</View>
	);
}
