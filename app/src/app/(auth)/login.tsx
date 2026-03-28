import { Link, useLocalSearchParams } from "expo-router";
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
import { LoginFormValues, loginSchema } from "@/lib/validations/auth";
import { client } from "@/api/client";

import { ErrorCode } from "@/api/error";
import { useAuthStore } from "@/stores/auth";

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
	} = useForm<LoginFormValues>({
		resolver: zodResolver(loginSchema),
		defaultValues: {
			email: "",
			password: "",
		},
	});

	const { message } = useLocalSearchParams<{ message?: string }>();

	const onSubmit = async (values: LoginFormValues) => {
		const deviceInfo = {
			device_name: Device.deviceName ?? null,
			model_name: Device.modelName ?? null,
			os: `${Platform.OS} ${Device.osVersion ?? ""}`.trim(),
		};

		const { data, error } = await client.POST("/auth/login/email", {
			body: {
				email: values.email,
				password: values.password,
				device_info: deviceInfo,
			},
		});

		if (error) {
			const code: ErrorCode = error.code;
			switch (code) {
				case "UNAUTHORIZED":
					setError("password", {
						message: "メールアドレスもしくはパスワードが正しくありません",
					});
					break;
				case "SESSION_CREATION_FAILED":
					setError("password", {
						message: "問題が発生しました。もう一度お試しください",
					});
					break;
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
				{message && (
					<View className="mb-10 flex-row items-center gap-1">
						<Text className="text-base text-green-700">
							新規登録に成功しました。ログインしてください
						</Text>
					</View>
				)}

				<Text className="text-3xl font-bold text-foreground">
					おかえりなさい！
				</Text>

				<View className="mt-10 gap-5">
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
									autoComplete="email"
									textContentType="emailAddress"
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

				{/* 登録ボタン */}
				<Button
					onPress={handleSubmit(onSubmit)}
					className="mt-10 h-12 w-full rounded-xl bg-main"
					disabled={isSubmitting}
				>
					<Text className="font-bold">
						{isSubmitting ? "ログイン中" : "ログイン"}
					</Text>
				</Button>

				{/* ログインリンク */}
				<View className="mt-6 flex-row items-center justify-center gap-1">
					<Text className="text-muted-foreground">
						アカウントをお持ちでない方は
					</Text>
					<Link href="/register" asChild>
						<Button variant="link" size="sm" className="px-0">
							<Text className="text-main">新規登録</Text>
						</Button>
					</Link>
				</View>
			</View>
		</KeyboardAvoidingView>
	);
}
