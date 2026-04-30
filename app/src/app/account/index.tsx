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
import { useAuthStatus, useCurrentUser } from "@/hooks/useCurrentUser";
import { useAuthMutation } from "@/hooks/useAuthMutation";
import { Button } from "@/components/ui/button";
import { useState } from "react";

export default function Account() {
	const { authStatus } = useAuthStatus();
	const { authGoogleLink, authGoogleUnlink, logout } = useAuthMutation();
	const [rootError, setRootError] = useState("");

	const onAuthGoogleLinkSubmit = () => {
		authGoogleLink.mutate(undefined, {
			onError: ({ error }) => {
				switch (error.code) {
					case "GOOGLE_AUTH_CANCELLED":
						return;
					case "GOOGLE_EMAIL_NOT_VERIFIED":
						setRootError("Googleのメールアドレスを認証してください");
						break;
					case "INVALID_GOOGLE_TOKEN":
						setRootError("エラーが発生しました。再度お試しください");
						break;
					case "EMAIL_ALREADY_IN_USE":
						setRootError(
							"このメールアドレスは既に別の方法で登録されています。別のログイン方法でログイン後、アカウント設定からリンクできます。",
						);
						break;
					case "GLOBAL_RATE_LIMITED":
						setRootError("操作が多すぎます。時間をおいて再度お試しください");
						break;
					default:
						setRootError(
							"エラーが発生しました。時間をおいて再度お試しください",
						);
				}
			},
			onSuccess: () => {
				setRootError("");
			},
		});
	};

	const onAuthGoogleUnlinkSubmit = () => {
		authGoogleUnlink.mutate(undefined, {
			onError: ({ error }) => {
				switch (error.code) {
					case "GOOGLE_AUTH_CANCELLED":
						return;
					case "EMAIL_AUTHENTICATION_REQUIRED":
						setRootError(
							"Googleログインを解除するには、メールアドレスログインを設定している必要があります。",
						);
						break;
					case "GLOBAL_RATE_LIMITED":
						setRootError("操作が多すぎます。時間をおいて再度お試しください");
						break;
					default:
						setRootError(
							"エラーが発生しました。時間をおいて再度お試しください",
						);
				}
			},
			onSuccess: () => {
				setRootError("");
			},
		});
	};

	return (
		<View className="flex-1 bg-white">
			{rootError !== "" && (
				<View className="mt-8 items-center mx-7">
					<Text className="text-base text-red-500">{rootError}</Text>
				</View>
			)}
			<View className="mt-11 mx-5 gap-9">
				<View className="flex-row items-center">
					<View className="flex-1 ml-5">
						<Text className="text-base font-bold">Googleログイン</Text>
						<Text className="text-sm text-gray-500 mt-1">
							{authStatus?.is_google_linked ? "設定済み" : "未設定"}
						</Text>
					</View>
					{authStatus?.is_google_linked ? (
						<Dialog>
							<DialogTrigger asChild>
								<Button
									variant="outline"
									size="lg"
									className="rounded-xl active:bg-gray-50"
									disabled={!authStatus.is_email_linked}
								>
									<Text className="font-medium text-foreground">解除する</Text>
								</Button>
							</DialogTrigger>

							<DialogContent
								showClose={false}
								className="w-[270px] p-0 gap-0 rounded-[14px] bg-white/95 overflow-hidden border-0"
							>
								<View className="pt-5 pb-4 px-4 items-center">
									<DialogTitle className="text-center text-[17px] font-semibold text-black mb-1">
										Googleログインを解除します
									</DialogTitle>
									<DialogDescription className="text-center text-[13px] text-gray-800">
										よろしいですか？
									</DialogDescription>
								</View>

								<View className="flex-row border-t border-gray-300/80 h-[44px]">
									<DialogClose asChild>
										<TouchableOpacity className="flex-1 justify-center items-center border-r border-gray-300/80">
											<Text className="text-gray-600 text-[17px]">
												キャンセル
											</Text>
										</TouchableOpacity>
									</DialogClose>

									<DialogClose asChild>
										<TouchableOpacity
											onPress={onAuthGoogleUnlinkSubmit}
											className="flex-1 justify-center items-center"
										>
											<Text className="text-rose-500 text-[17px] font-semibold">
												解除する
											</Text>
										</TouchableOpacity>
									</DialogClose>
								</View>
							</DialogContent>
						</Dialog>
					) : (
						<Button
							onPress={onAuthGoogleLinkSubmit}
							variant="outline"
							size="lg"
							className="rounded-xl active:bg-gray-50"
							disabled={authStatus?.is_google_linked}
						>
							<Text className="font-medium text-foreground">設定する</Text>
						</Button>
					)}
				</View>

				<Pressable
					onPress={() => router.push("/account/email")}
					className="flex-row items-center"
				>
					<View className="flex-1 ml-5">
						<Text className="text-base font-bold">メールアドレスログイン</Text>
						<Text className="text-sm text-gray-500 mt-1">
							{authStatus?.is_email_linked ? "設定済み" : "未設定"}
						</Text>
					</View>
					<ChevronRight color="#aaa" />
				</Pressable>

				<Dialog>
					<DialogTrigger asChild>
						<TouchableOpacity className="flex-row items-center py-4">
							<View className="ml-5">
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
