import { Image, Platform, View } from "react-native";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { Link, router } from "expo-router";
import {
	GoogleSignin,
	isErrorWithCode,
	isSuccessResponse,
	statusCodes,
} from "@react-native-google-signin/google-signin";

import * as Device from "expo-device";
import { client } from "@/api/client";
import { saveAccessToken, saveRefreshToken } from "@/lib/auth-storage";
import { useAuthStore } from "@/stores/auth";
import { useState } from "react";

export default function Welcome() {
	const [googleError, setGoogleError] = useState(false);

	const googleLogin = async () => {
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
					device_info: {
						device_name: Device.deviceName ?? null,
						model_name: Device.modelName ?? null,
						os: `${Platform.OS} ${Device.osVersion ?? ""}`.trim(),
					},
				},
			});

			if (error) {
				if (error.code === "SESSION_CREATION_FAILED") {
					router.replace({
						pathname: "/login",
						params: { message: "登録が完了しました。ログインしてください" },
					});
				} else {
					setGoogleError(true);
				}
				return;
			}

			await saveAccessToken(data.access_token);
			await saveRefreshToken(data.refresh_token);
			useAuthStore.getState().setCurrentUser(data.user);
			router.replace("/");
		} catch (error) {
			if (
				isErrorWithCode(error) &&
				error.code === statusCodes.SIGN_IN_CANCELLED
			) {
				return;
			}
			setGoogleError(true);
		}
	};

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
				<Button
					onPress={googleLogin}
					size={"lg"}
					className="h-12 rounded-xl bg-[#534AB7]"
				>
					<Text className="text-white text-base font-semibold">
						Googleで続ける
					</Text>
				</Button>
				{googleError && (
					<View className="flex-row justify-center items-center gap-2">
						<Text className="text-sm text-red-500">
							エラーが発生しました。再度お試しください
						</Text>
					</View>
				)}

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
