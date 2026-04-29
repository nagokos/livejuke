import {
	KeyboardAvoidingView,
	Platform,
	Pressable,
	TextInput,
	View,
} from "react-native";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { Input } from "@/components/ui/input";
import { useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { valibotResolver } from "@hookform/resolvers/valibot";
import { SendCodeFormValues, sendCodeSchema } from "@/lib/validations/auth";
import { X } from "lucide-react-native";
import { GoogleIcon } from "@/components/icons/google";
import { useAuthMutation } from "@/hooks/useAuthMutation";

const RATE_LIMIT_DURATION = 60000;

export default function Welcome() {
	const [step, setStep] = useState<"email" | "code">("email");
	const [code, setCode] = useState("");
	const [cooldown, setCooldown] = useState(0);
	const [rateLimited, setRateLimited] = useState(false);
	const [rootError, setRootError] = useState("");
	const [codeError, setCodeError] = useState("");
	const [googleError, setGoogleError] = useState("");

	const { sendCode, verifyCode, authGoogle, isProcessing } = useAuthMutation();

	const {
		control,
		handleSubmit,
		getValues,
		setError,
		formState: { errors },
	} = useForm<SendCodeFormValues>({
		resolver: valibotResolver(sendCodeSchema),
		defaultValues: { email: "" },
	});

	useEffect(() => {
		if (cooldown <= 0) return;
		const timer = setInterval(() => {
			setCooldown((prev) => prev - 1);
		}, 1000);
		return () => clearInterval(timer);
	}, [cooldown]);

	const onSendCodeSubmit = (values: SendCodeFormValues) => {
		setRootError("");
		sendCode.mutate(values, {
			onSuccess: (data) => {
				if (!data) return;
				setCooldown(data.resend_cooldown_seconds);
				setStep("code");
			},
			onError: ({ error }) => {
				const errorCode = error?.code;
				switch (errorCode) {
					case "INVALID_EMAIL":
						setError("email", {
							message: "メールアドレスを正しく入力してください",
						});
						break;
					case "SEND_CODE_RATE_LIMITED":
						setRateLimited(true);
						setTimeout(() => setRateLimited(false), RATE_LIMIT_DURATION);
						setRootError("操作が多すぎます。時間をおいて再度お試しください");
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
		});
	};

	const onVerifyCodeSubmit = () => {
		if (!code) {
			setCodeError("認証コードを入力してください");
			return;
		}
		setCodeError("");
		setRootError("");

		verifyCode.mutate(
			{ email: getValues("email"), code },
			{
				onError: ({ error }) => {
					const errorCode = error?.code;
					switch (errorCode) {
						case "INVALID_EMAIL":
							setError("email", {
								message: "メールアドレスを正しく入力してください",
							});
							break;
						case "INVALID_VERIFICATION_CODE":
							setCodeError("認証コードが正しくありません");
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
			},
		);
	};

	const onAuthGoogleSubmit = () => {
		setGoogleError("");

		authGoogle.mutate(undefined, {
			onError: ({ error }) => {
				const errorCode = error?.code;
				switch (errorCode) {
					case "GOOGLE_AUTH_CANCELLED":
						return;
					case "GOOGLE_EMAIL_NOT_VERIFIED":
						setGoogleError("Googleのメールアドレスを認証してください");
						break;
					case "INVALID_GOOGLE_TOKEN":
						setGoogleError("エラーが発生しました。再度お試しください");
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
		});
	};

	const resetToEmail = (onChange: (value: string) => void) => {
		onChange("");
		setCode("");
		setStep("email");
		setRootError("");
		setCodeError("");
		sendCode.reset();
		verifyCode.reset();
	};

	return (
		<KeyboardAvoidingView
			className="flex-1 bg-white"
			behavior={Platform.OS === "ios" ? "padding" : "height"}
		>
			<View className="flex-1 mt-7 justify-center px-6">
				{rootError !== "" && (
					<View className="mb-8">
						<Text className="text-base text-red-500">{rootError}</Text>
					</View>
				)}

				<Text className="text-3xl font-bold text-foreground">
					LiveJukeへようこそ！
				</Text>

				<Button
					onPress={onAuthGoogleSubmit}
					variant="outline"
					className="mt-8 h-12 w-full rounded-xl active:bg-gray-50"
					disabled={isProcessing}
				>
					<GoogleIcon />
					<Text className="text-base font-medium text-foreground">
						{isProcessing ? "読み込み中..." : "Googleで続ける"}
					</Text>
				</Button>
				{googleError !== "" && (
					<View className="items-center mt-2">
						<Text className="text-sm text-red-500">{googleError}</Text>
					</View>
				)}

				<View className="mt-9 flex-row items-center">
					<View className="flex-1 h-px bg-gray-200" />
				</View>

				<View className="mt-8 gap-5">
					<Controller
						name="email"
						control={control}
						render={({ field: { onChange, value } }) => (
							<View className="gap-2">
								<Text className="text-sm font-medium text-foreground">
									メールアドレス
								</Text>
								<View className="relative">
									<Input
										value={value}
										editable={step === "email"}
										onChangeText={onChange}
										keyboardType="email-address"
										autoCapitalize="none"
										autoCorrect={false}
										textContentType="emailAddress"
										placeholder="example@email.com"
										selectionColor="#534AB7"
										placeholderTextColor="#9CA3AF"
										className="h-12 rounded-lg border border-gray-200 bg-gray-50 pl-4 pr-10 focus:border-2 focus:border-main"
									/>
									{value !== "" && (
										<Pressable
											onPress={() => resetToEmail(onChange)}
											className="absolute right-3 top-[13px] w-[22px] h-[22px] items-center justify-center"
										>
											<X size={15} strokeWidth={3} />
										</Pressable>
									)}
								</View>
								{errors.email && (
									<Text className="text-sm text-red-500">
										{errors.email.message}
									</Text>
								)}
							</View>
						)}
					/>

					{step === "code" && (
						<View className="gap-2">
							<Text className="text-sm font-medium text-foreground">
								認証コード
							</Text>
							<TextInput
								value={code}
								onChangeText={setCode}
								placeholder="コードを入力"
								autoCapitalize="none"
								autoCorrect={false}
								textContentType="oneTimeCode"
								className="h-12 rounded-lg border border-gray-200 bg-gray-50 pl-4 pr-10 focus:border-2 focus:border-main"
							/>
							{codeError !== "" && (
								<Text className="text-sm text-red-500">{codeError}</Text>
							)}
						</View>
					)}
				</View>

				<Button
					onPress={
						step === "email"
							? handleSubmit(onSendCodeSubmit)
							: onVerifyCodeSubmit
					}
					className="mt-10 h-12 w-full rounded-xl bg-main active:opacity-80"
					disabled={isProcessing || rateLimited}
				>
					<Text className="font-bold">
						{isProcessing
							? step === "email"
								? "送信中..."
								: "確認中..."
							: step === "email"
								? "認証コードを送信"
								: "続ける"}
					</Text>
				</Button>

				{step === "code" && (
					<View className="mt-4 items-center">
						{cooldown > 0 ? (
							<Text className="text-gray-400 text-sm">
								{`コードを再送する (${cooldown}s)`}
							</Text>
						) : (
							<Button
								onPress={() => onSendCodeSubmit({ email: getValues("email") })}
								disabled={isProcessing || rateLimited}
								size="sm"
								variant="ghost"
								className="rounded-xl active:bg-main/10"
							>
								<Text className="!text-main text-sm">コードを再送する</Text>
							</Button>
						)}
					</View>
				)}
			</View>
		</KeyboardAvoidingView>
	);
}
