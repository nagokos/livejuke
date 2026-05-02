import { Pressable, TouchableOpacity, View } from "react-native";
import { Text } from "@/components/ui/text";
import {
	ChevronRight,
	MessageCircleQuestionMark,
	UserRound,
	LogOut,
} from "lucide-react-native";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { router } from "expo-router";
import { Image } from "expo-image";
import { useCurrentUser } from "@/hooks/useCurrentUser";
import {
	Dialog,
	DialogClose,
	DialogContent,
	DialogDescription,
	DialogTitle,
	DialogTrigger,
} from "../ui/dialog";
import { useAuthMutation } from "@/hooks/useAuthMutation";

export default function Settings() {
	const { currentUser } = useCurrentUser();
	const { logout } = useAuthMutation();

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
								source={{ uri: currentUser?.avatar_url ?? undefined }}
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
			<View className="mt-10 mx-5 gap-14">
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

				<Dialog>
					<DialogTrigger asChild>
						<TouchableOpacity className="flex-row items-center">
							<View className="w-10 items-center justify-center">
								<LogOut />
							</View>
							<View className="flex-1 flex-row items-center ml-5">
								<Text className="text-base font-bold  flex-1">ログアウト</Text>
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
