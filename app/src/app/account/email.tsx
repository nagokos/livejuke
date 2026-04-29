import { View, Text, TextInput, Pressable } from "react-native";
import { Button } from "@/components/ui/button";
import { useCallback, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { valibotResolver } from "@hookform/resolvers/valibot";
import { SendCodeFormValues, sendCodeSchema } from "@/lib/validations/auth";
import { Input } from "@/components/ui/input";
import { X } from "lucide-react-native";
import { useCurrentUser } from "@/hooks/useCurrentUser";
import { useAuthMutation } from "@/hooks/useAuthMutation";

const RATE_LIMIT_DURATION = 60000;

export default function EmailEdit() {
	const { currentUser } = useCurrentUser();

	const [step, setStep] = useState<"email" | "code">("email");
	const [code, setCode] = useState("");
	const [cooldown, setCooldown] = useState(0);
	const [rateLimited, setRateLimited] = useState(false);
	const [rootError, setRootError] = useState("");
	const [codeError, setCodeError] = useState("");
	const { sendCode, upsertEmail, isProcessing } = useAuthMutation();

	useEffect(() => {
		if (cooldown <= 0) return;
		const timer = setInterval(() => {
			setCooldown((prev) => prev - 1);
		}, 1000);
		return () => clearInterval(timer);
	}, [cooldown]);

	const {
		control,
		handleSubmit,
		getValues,
		setValue,
		setError,
		formState: { errors, isSubmitting },
	} = useForm<SendCodeFormValues>({
		resolver: valibotResolver(sendCodeSchema),
		defaultValues: { email: "" },
	});

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

	const onUpsertEmail = () => {
		if (!code) {
			setCodeError("認証コードを入力してください");
			return;
		}
		setCodeError("");
		setRootError("");

		upsertEmail.mutate(
			{ email: getValues("email"), code },
			{
				onError: ({ error }) => {
					console.log(error);
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
				onSuccess: () => {
					setValue("email", "");
					setCode("");
					setStep("email");
					setRootError("");
					setCodeError("");
				},
			},
		);
	};

	const resetToEmail = useCallback((onChange: (value: string) => void) => {
		onChange("");
		setCode("");
		setStep("email");
		setRootError("");
		setCodeError("");
	}, []);

	return (
		<View className="flex-1 bg-white px-6 justify-between">
			<View className="mt-10 gap-6">
				{rootError !== "" && (
					<View>
						<Text className="text-base text-red-500">{rootError}</Text>
					</View>
				)}
				<View className="gap-1">
					<Text className="text-sm text-gray-500">現在のメールアドレス</Text>
					<Text className="text-lg font-medium">{currentUser?.email}</Text>
				</View>

				<Controller
					name="email"
					control={control}
					render={({ field: { onChange, value } }) => (
						<View className="gap-2">
							<Text className="text-sm font-medium text-foreground">
								新規メールアドレス
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

			<View className="mb-10">
				<Button
					onPress={handleSubmit(
						step === "email" ? onSendCodeSubmit : onUpsertEmail,
					)}
					className="h-12 w-full rounded-xl bg-main active:opacity-80"
					disabled={isProcessing || rateLimited}
				>
					<Text className="font-bold text-white">
						{isSubmitting
							? step === "email"
								? "送信中..."
								: "確認中..."
							: step === "email"
								? "認証コードを送信"
								: "メールアドレスを更新"}
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
								onPress={() => onSendCodeSubmit}
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
		</View>
	);
}
