import { Platform } from "react-native";
import { client } from "@/api/client";
import { saveAccessToken, saveRefreshToken } from "@/lib/auth-storage";
import { components } from "@/types/schema";
import {
	GoogleSignin,
	isErrorWithCode,
	isSuccessResponse,
	statusCodes,
} from "@react-native-google-signin/google-signin";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { router } from "expo-router";

import * as Device from "expo-device";
import { AppErrorWithResponse } from "@/types/api";
import { useAuthStore } from "@/stores/auth";

type AuthResponse = components["schemas"]["AuthResponse"];
type SendCodeParams = components["schemas"]["SendCodeInput"];
type SendCodeResponse = components["schemas"]["SendCodeResponse"];
type VerifyCodeParams = Omit<
	components["schemas"]["VerifyCodeInput"],
	"device_info"
>;
type CurrentUserResponse = components["schemas"]["CurrentUserResponse"];
type UpdateEmailParams = components["schemas"]["UpdateEmailInput"];

const getDeviceInfo = () => ({
	device_name: Device.deviceName ?? null,
	model_name: Device.modelName ?? null,
	os: `${Platform.OS} ${Device.osVersion ?? ""}`.trim(),
});

export const useAuthMutation = () => {
	const queryClient = useQueryClient();
	const logout = useAuthStore((state) => state.logout);
	const setHasToken = useAuthStore((state) => state.setHasToken);

	const refreshCurrentUser = async () => {
		await queryClient.invalidateQueries({
			queryKey: ["currentUser"],
		});
	};

	const handleAuthSuccess = async (data: AuthResponse) => {
		await saveAccessToken(data.access_token);
		await saveRefreshToken(data.refresh_token);
		setHasToken();

		try {
			await queryClient.fetchQuery({
				queryKey: ["currentUser"],
				queryFn: async () => {
					const { data, error, response } = await client.GET("/me");
					if (error) throw { error, response };
					return data;
				},
				staleTime: Infinity,
				retry: false,
			});

			router.replace("/");
		} catch (e) {
			await logout();
			throw e;
		}
	};

	const sendCode = useMutation<
		SendCodeResponse,
		AppErrorWithResponse,
		SendCodeParams
	>({
		mutationFn: async (params) => {
			const { data, error, response } = await client.POST(
				"/auth/email/send-code",
				{
					body: params,
				},
			);
			if (error) throw { error, response };
			return data;
		},
	});

	const verifyCode = useMutation<
		AuthResponse,
		AppErrorWithResponse,
		VerifyCodeParams
	>({
		mutationFn: async (params) => {
			const { data, error, response } = await client.POST(
				"/auth/email/verify-code",
				{
					body: { ...params, device_info: getDeviceInfo() },
				},
			);
			if (error) throw { error, response };
			return data;
		},
		onSuccess: handleAuthSuccess,
	});

	const authGoogle = useMutation<AuthResponse, AppErrorWithResponse>({
		mutationFn: async () => {
			let googleResponse;

			try {
				googleResponse = await GoogleSignin.signIn();
			} catch (err) {
				if (isErrorWithCode(err) && err.code === statusCodes.SIGN_IN_CANCELLED)
					throw {
						error: {
							code: "GOOGLE_AUTH_CANCELLED",
							message: "cancel google authentication",
						},
					};

				console.error("Google SDK Error:", err);
				throw {
					error: {
						code: "GOOGLE_SDK_ERROR",
						message: "internal google authentication error",
					},
				};
			}

			if (!isSuccessResponse(googleResponse))
				throw {
					error: {
						code: "GOOGLE_AUTH_CANCELLED",
						message: "cancel google authentication",
					},
				};

			if (!googleResponse.data.idToken)
				throw {
					error: {
						code: "GOOGLE_AUTH_NO_TOKEN",
						message: "no token google authentication",
					},
				};

			const idToken = googleResponse.data.idToken;

			const { data, error, response } = await client.POST("/auth/google", {
				body: { id_token: idToken, device_info: getDeviceInfo() },
			});

			if (error) throw { error, response };

			return data;
		},
		onSuccess: handleAuthSuccess,
	});

	const upsertEmail = useMutation<
		CurrentUserResponse,
		AppErrorWithResponse,
		UpdateEmailParams
	>({
		mutationFn: async (params) => {
			const { data, error, response } = await client.PATCH("/auth/email", {
				body: params,
			});

			if (error) throw { error, response };

			return data;
		},
		onSuccess: refreshCurrentUser,
	});

	return {
		sendCode,
		verifyCode,
		authGoogle,
		upsertEmail,
		isProcessing:
			sendCode.isPending ||
			verifyCode.isPending ||
			authGoogle.isPending ||
			upsertEmail.isPending,
	};
};
