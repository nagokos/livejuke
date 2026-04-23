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
import { useEffect, useState, useCallback } from "react";
import { Controller, useForm } from "react-hook-form";
import { valibotResolver } from "@hookform/resolvers/valibot";
import { SendCodeFormValues, sendCodeSchema } from "@/lib/validations/auth";
import { ErrorCode } from "@/api/error";
import { useAuthStore } from "@/stores/auth";
import { client } from "@/api/client";
import { router } from "expo-router";
import * as Device from "expo-device";
import { saveAccessToken, saveRefreshToken } from "@/lib/auth-storage";
import { X } from "lucide-react-native";
import {
	GoogleSignin,
	isErrorWithCode,
	isSuccessResponse,
	statusCodes,
} from "@react-native-google-signin/google-signin";
import type { components } from "@/types/schema";
import { GoogleIcon } from "@/components/icons/google";

type AuthResponse = components["schemas"]["AuthResponse"];

const RATE_LIMIT_DURATION = 60000;

const getDeviceInfo = () => ({
	device_name: Device.deviceName ?? null,
	model_name: Device.modelName ?? null,
	os: `${Platform.OS} ${Device.osVersion ?? ""}`.trim(),
});

export default function Welcome() {
	const [step, setStep] = useState<"email" | "code">("email");
	const [code, setCode] = useState("");
	const [cooldown, setCooldown] = useState(0);
	const [rateLimited, setRateLimited] = useState(false);
	const [rootError, setRootError] = useState("");
	const [codeError, setCodeError] = useState("");
	const [googleError, setGoogleError] = useState(false);

	const {
		control,
		handleSubmit,
		getValues,
		setError,
		formState: { errors, isSubmitting },
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

	const handleAuthSuccess = useCallback(async (data: AuthResponse) => {
		await saveAccessToken(data.access_token);
		await saveRefreshToken(data.refresh_token);
		useAuthStore.getState().setCurrentUser(data.user);
		router.replace("/");
	}, []);

	const sendCode = useCallback(
		async (email: string) => {
			setRootError("");
			const { data, error, response } = await client.POST(
				"/auth/email/send-code",
				{ body: { email } },
			);

			if (error) {
				const errorCode: ErrorCode = error.code;
				switch (errorCode) {
					case "INVALID_EMAIL":
						setError("email", {
							message: "メールアドレスを正しく入力してください",
						});
						break;
					case "RATE_LIMIT_EXCEEDED":
						if (response.status === 429) {
							setRateLimited(true);
							setTimeout(() => setRateLimited(false), RATE_LIMIT_DURATION);
						}
						setRootError(
							"エラーが発生しました。時間をおいて再度お試しください",
						);
						break;
				}
				return null;
			}

			return data;
		},
		[setError],
	);

	const onSendCode = useCallback(
		async (values: SendCodeFormValues) => {
			const data = await sendCode(values.email);
			if (!data) return;
			setCooldown(data.resend_cooldown_seconds);
			setStep("code");
		},
		[sendCode],
	);

	const onResend = useCallback(async () => {
		const data = await sendCode(getValues("email"));
		if (!data) return;
		setCooldown(data.resend_cooldown_seconds);
	}, [sendCode, getValues]);

	const onVerifyCode = useCallback(async () => {
		if (!code) {
			setCodeError("認証コードを入力してください");
			return;
		}
		setCodeError("");
		setRootError("");

		const email = getValues("email");
		const { data, error } = await client.POST("/auth/email/verify-code", {
			body: {
				email,
				code,
				device_info: getDeviceInfo(),
			},
		});

		if (error) {
			const errorCode: ErrorCode = error.code;
			switch (errorCode) {
				case "INVALID_EMAIL":
					setError("email", {
						message: "メールアドレスを正しく入力してください",
					});
					break;
				case "INVALID_VERIFICATION_CODE":
					setCodeError("認証コードが正しくありません");
					break;
				case "RATE_LIMIT_EXCEEDED":
					setRootError("操作が多すぎます。時間をおいて再度お試しください");
					break;
				default:
					setRootError("エラーが発生しました。時間をおいて再度お試しください");
			}
			return;
		}

		await handleAuthSuccess(data);
	}, [code, getValues, setError, handleAuthSuccess]);

	const onGoogleLogin = useCallback(async () => {
		setGoogleError(false);
		try {
			const response = await GoogleSignin.signIn();
			if (!isSuccessResponse(response)) return;

			const idToken = response.data.idToken;
			if (!idToken) {
				setGoogleError(true);
				return;
			}

			const { data, error } = await client.POST("/auth/google", {
				body: {
					id_token: idToken,
					device_info: getDeviceInfo(),
				},
			});

			if (error) {
				setGoogleError(true);
				return;
			}

			await handleAuthSuccess(data);
		} catch (error) {
			if (
				isErrorWithCode(error) &&
				error.code === statusCodes.SIGN_IN_CANCELLED
			) {
				return;
			}
			setGoogleError(true);
		}
	}, [handleAuthSuccess]);

	const resetToEmail = useCallback((onChange: (value: string) => void) => {
		onChange("");
		setCode("");
		setStep("email");
		setRootError("");
		setCodeError("");
	}, []);

	return (
		<KeyboardAvoidingView
			className="flex-1 bg-white"
			behavior={Platform.OS === "ios" ? "padding" : "height"}
		>
			<View className="flex-1 justify-center px-6">
				{rootError !== "" && (
					<View className="mb-8">
						<Text className="text-base text-red-500">{rootError}</Text>
					</View>
				)}

				<Text className="text-3xl font-bold text-foreground">
					LiveJukeへようこそ！
				</Text>

				<Button
					onPress={onGoogleLogin}
					variant="outline"
					className="mt-8 h-12 w-full rounded-xl active:bg-gray-50"
				>
					<GoogleIcon />
					<Text className="text-base font-medium text-foreground">
						Googleで続ける
					</Text>
				</Button>
				{googleError && (
					<View className="items-center mt-2">
						<Text className="text-sm text-red-500">
							エラーが発生しました。再度お試しください
						</Text>
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
					onPress={handleSubmit(step === "email" ? onSendCode : onVerifyCode)}
					className="mt-10 h-12 w-full rounded-xl bg-main active:opacity-80"
					disabled={isSubmitting || rateLimited}
				>
					<Text className="font-bold">
						{isSubmitting
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
								onPress={onResend}
								disabled={rateLimited}
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
