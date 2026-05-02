import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { TextInput, TouchableOpacity, View } from "react-native";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useEffect, useState } from "react";
import { useUserMutation } from "@/hooks/useUserMutation";
import { router } from "expo-router";

const RATE_LIMIT_DURATION = 60000;

export default function AccountDelete() {
	const [cooldown, setCooldown] = useState(0);
	const [code, setCode] = useState("");
	const [rateLimited, setRateLimited] = useState(false);
	const [codeError, setCodeError] = useState("");

	const { sendCode, deleteUser, isProcessing } = useUserMutation();

	useEffect(() => {
		if (cooldown <= 0) return;
		const timer = setInterval(() => {
			setCooldown((prev) => prev - 1);
		}, 1000);
		return () => clearInterval(timer);
	}, [cooldown]);

	const onSendCodeSubmit = () => {
		setCodeError("");
		sendCode.mutate(undefined, {
			onSuccess: (data) => {
				setCooldown(data.resend_cooldown_seconds);
			},
			onError: ({ error }) => {
				const errorCode = error?.code;
				switch (errorCode) {
					case "SEND_CODE_RATE_LIMITED":
						setRateLimited(true);
						setTimeout(() => setRateLimited(false), RATE_LIMIT_DURATION);
						setCodeError("操作が多すぎます。時間をおいて再度お試しください");
						break;
					case "GLOBAL_RATE_LIMITED":
						setCodeError("操作が多すぎます。時間をおいて再度お試しください");
						break;
					default:
						setCodeError(
							"エラーが発生しました。時間をおいて再度お試しください",
						);
				}
			},
		});
	};

	const onDeleteUserSubmit = () => {
		if (!code) {
			setCodeError("認証コードを入力してください");
			return;
		}
		setCodeError("");

		deleteUser.mutate(
			{ code },
			{
				onError: ({ error }) => {
					console.log(error);
					const errorCode = error?.code;
					switch (errorCode) {
						case "INVALID_VERIFICATION_CODE":
							setCodeError("認証コードが正しくありません");
							break;
						case "GLOBAL_RATE_LIMITED":
							setCodeError("操作が多すぎます。時間をおいて再度お試しください");
							break;
						default:
							setCodeError(
								"エラーが発生しました。時間をおいて再度お試しください",
							);
					}
				},
				onSuccess: () => {
					router.replace("/(tabs)");
				},
			},
		);
	};

	return (
		<View className="flex-1 bg-white px-8 justify-between">
			<View className="mt-9 gap-2">
				<Text className="font-bold text-lg">アカウントが削除されます</Text>
				<View className="mt-3 gap-3">
					<Text className="text-gray-800 text-sm">
						一度アカウントを削除すると、
						<Text className="text-rose-500 text-sm">
							復元することはできません。
						</Text>
					</Text>
					<Text className="text-gray-800 text-sm">
						アカウント削除を押すとメールアドレス宛に確認コードが届きます。
					</Text>
					<Text className="text-gray-800 text-sm">
						確認コードを入力するとアカウントが削除されます。
					</Text>
					<Text className="text-gray-800 text-sm">
						サービスについてのご質問、ご要望がある方はお問い合わせをご利用ください。
					</Text>
				</View>
			</View>
			<Dialog>
				<DialogTrigger asChild>
					<TouchableOpacity
						onPress={onSendCodeSubmit}
						className="mb-10 h-12 rounded-xl justify-center items-center"
					>
						<Text className="text-rose-500  font-bold text-sm">
							アカウント削除
						</Text>
					</TouchableOpacity>
				</DialogTrigger>
				<DialogContent className="sm:max-w-[425px]">
					<DialogHeader>
						<DialogTitle>アカウント削除</DialogTitle>
						<DialogDescription className="text-gray-700">
							確認コードを入力してください。アカウント削除後は復元できません。
						</DialogDescription>
					</DialogHeader>
					<View className="grid gap-4">
						<View className="grid gap-3">
							<Text className="text-sm">確認コード</Text>
							<TextInput
								value={code}
								onChangeText={setCode}
								placeholder="コードを入力"
								autoCapitalize="none"
								autoCorrect={false}
								textContentType="oneTimeCode"
								className="h-12 rounded-lg border border-gray-200 bg-gray-50 pl-4 pr-10 "
							/>
							{codeError !== "" && (
								<Text className="text-sm text-red-500">{codeError}</Text>
							)}
						</View>
					</View>
					<DialogFooter className="mt-3">
						<Button
							onPress={onDeleteUserSubmit}
							disabled={isProcessing || rateLimited}
							className="bg-rose-500 active:bg-rose-600"
						>
							<Text className="font-bold">アカウント削除</Text>
						</Button>
					</DialogFooter>
					<View className="items-center mt-2 mb-1">
						{cooldown > 0 ? (
							<Text className="text-gray-400 text-sm">
								{`コードを再送する (${cooldown}s)`}
							</Text>
						) : (
							<TouchableOpacity
								onPress={onSendCodeSubmit}
								disabled={isProcessing || rateLimited}
							>
								<Text className="!text-rose-500 text-sm">コードを再送する</Text>
							</TouchableOpacity>
						)}
					</View>
				</DialogContent>
			</Dialog>
		</View>
	);
}
