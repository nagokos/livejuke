import { Link } from "expo-router";
import {
	KeyboardAvoidingView,
	Platform,
	Pressable,
	TextInput,
	View,
} from "react-native";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { Eye, EyeOff } from "lucide-react-native";
import { useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { RegisterFormValues, registerSchema } from "@/lib/validations/auth";
import { ErrorCode } from "@/api/error";
import { useAuthStore } from "@/stores/auth";
import { client } from "@/api/client";
import { router } from "expo-router";
import * as Device from "expo-device";
import { saveAccessToken, saveRefreshToken } from "@/lib/auth-storage";

export default function RegisterScreen() {
	const [showPassword, setShowPassword] = useState(false);
	const {
		control,
		handleSubmit,
		setError,
		formState: { errors, isSubmitting },
	} = useForm<RegisterFormValues>({
		resolver: zodResolver(registerSchema),
		defaultValues: {
			display_name: "",
			email: "",
			password: "",
		},
	});

	const onSubmit = async (values: RegisterFormValues) => {
		const deviceInfo = {
			device_name: Device.deviceName ?? null,
			model_name: Device.modelName ?? null,
			os: `${Platform.OS} ${Device.osVersion ?? ""}`.trim(),
		};

		const { data, error } = await client.POST("/auth/register/email", {
			body: {
				display_name: values.display_name,
				email: values.email,
				password: values.password,
				device_info: deviceInfo,
			},
		});

		if (error) {
			const code: ErrorCode = error.code;

			switch (code) {
				case "EMAIL_ALREADY_EXISTS":
					setError("email", {
						message: "このメールアドレスは既に使われています",
					});
					break;
				case "INVALID_EMAIL":
					setError("email", {
						message: "メールアドレスを正しく入力してください",
					});
					break;
				case "INVALID_PASSWORD":
					setError("password", {
						message: "パスワードを正しく入力してください",
					});
					break;
				case "SESSION_CREATION_FAILED":
					router.replace({
						pathname: "/login",
						params: { message: "登録が完了しました。ログインしてください" },
					});
					return;
				case "RATE_LIMIT_EXCEEDED":
					setError("root", {
						message: "エラーが発生しました。時間をおいて再度お試しください",
					});
					break;
			}

			return;
		}

		saveAccessToken(data.access_token);
		saveRefreshToken(data.refresh_token);
		useAuthStore.getState().setCurrentUser(data.user);
		router.replace("/");
	};

	return (
		<KeyboardAvoidingView
			className="flex-1 bg-white"
			behavior={Platform.OS === "ios" ? "padding" : "height"}
		>
			<View className="flex-1 justify-center px-6">
				{errors.root && (
					<View className="mb-8 flex-row items-center gap-2">
						<Text className="text-base text-red-500">
							{errors.root.message}
						</Text>
					</View>
				)}

				<Text className="text-3xl font-bold text-foreground">
					LiveJukeへようこそ！
				</Text>

				<View className="mt-10 gap-5">
					<Controller
						name="display_name"
						control={control}
						render={({ field: { onChange, value } }) => (
							<View className="gap-2">
								<Text className="text-sm font-medium text-foreground">
									表示名
								</Text>
								<TextInput
									value={value}
									onChangeText={onChange}
									autoCapitalize="none"
									autoComplete="off"
									autoCorrect={false}
									textContentType="none"
									maxLength={30}
									placeholder="表示名を入力"
									selectionColor="#534AB7"
									placeholderTextColor="#9CA3AF"
									className="h-12 rounded-lg border border-gray-200 bg-gray-50 px-4 text-base text-foreground"
								/>
								{errors.display_name && (
									<Text className="text-sm text-red-500">
										{errors.display_name.message}
									</Text>
								)}
							</View>
						)}
					/>

					<Controller
						name="email"
						control={control}
						render={({ field: { onChange, value } }) => (
							<View className="gap-2">
								<Text className="text-sm font-medium text-foreground">
									メールアドレス
								</Text>
								<TextInput
									value={value}
									onChangeText={onChange}
									keyboardType="email-address"
									autoCapitalize="none"
									autoCorrect={false}
									textContentType="emailAddress"
									placeholder="example@email.com"
									selectionColor="#534AB7"
									placeholderTextColor="#9CA3AF"
									className="h-12 rounded-lg border border-gray-200 bg-gray-50 px-4 text-base text-foreground"
								/>
								{errors.email && (
									<Text className="text-sm text-red-500">
										{errors.email.message}
									</Text>
								)}
							</View>
						)}
					/>

					<Controller
						name="password"
						control={control}
						render={({ field: { onChange, value } }) => (
							<View className="gap-2">
								<Text className="text-sm font-medium text-foreground">
									パスワード
								</Text>
								<View className="flex-row items-center h-12 rounded-lg border border-gray-200 bg-gray-50 px-4">
									<TextInput
										value={value}
										onChangeText={onChange}
										autoCapitalize="none"
										autoCorrect={false}
										maxLength={128}
										secureTextEntry={!showPassword}
										placeholder="8文字以上"
										selectionColor="#534AB7"
										placeholderTextColor="#9CA3AF"
										className="flex-1 text-base text-foreground"
									/>
									<Pressable onPress={() => setShowPassword(!showPassword)}>
										{showPassword ? (
											<Eye size={20} className="text-gray-400" />
										) : (
											<EyeOff size={20} className="text-gray-400" />
										)}
									</Pressable>
								</View>
								{errors.password && (
									<Text className="text-sm text-red-500">
										{errors.password.message}
									</Text>
								)}
							</View>
						)}
					/>
				</View>

				<Button
					onPress={handleSubmit(onSubmit)}
					className="mt-10 h-12 w-full rounded-xl bg-main"
					disabled={isSubmitting}
				>
					<Text className="font-bold">
						{isSubmitting ? "登録中" : "登録する"}
					</Text>
				</Button>

				<View className="mt-6 flex-row items-center justify-center gap-1">
					<Text className="text-muted-foreground">
						アカウントをお持ちの方は
					</Text>
					<Link href="/login" asChild>
						<Button variant="link" size="sm" className="px-0">
							<Text className="text-main">ログイン</Text>
						</Button>
					</Link>
				</View>
			</View>
		</KeyboardAvoidingView>
	);
}
