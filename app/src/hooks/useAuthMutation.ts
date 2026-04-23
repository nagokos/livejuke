import { Platform } from "react-native";
import { client } from "@/api/client";
import {
	getRefreshToken,
	saveAccessToken,
	saveRefreshToken,
} from "@/lib/auth-storage";
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
	const zustandLogout = useAuthStore((state) => state.logout);
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
			await zustandLogout();
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

	const getIdToken = async () => {
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
		return idToken;
	};

	const authGoogle = useMutation<AuthResponse, AppErrorWithResponse, undefined>(
		{
			mutationFn: async () => {
				const idToken = await getIdToken();
				const { data, error, response } = await client.POST("/auth/google", {
					body: { id_token: idToken, device_info: getDeviceInfo() },
				});
				if (error) throw { error, response };
				return data;
			},
			onSuccess: handleAuthSuccess,
		},
	);

	const authGoogleLink = useMutation<
		undefined,
		AppErrorWithResponse,
		undefined
	>({
		mutationFn: async () => {
			const idToken = await getIdToken();
			const { error, response } = await client.POST("/auth/google/link", {
				body: { id_token: idToken },
			});
			if (error) throw { error, response };
			return;
		},
		onSuccess: refreshCurrentUser,
	});

	const authGoogleUnlink = useMutation<
		undefined,
		AppErrorWithResponse,
		undefined
	>({
		mutationFn: async () => {
			const { error, response } = await client.DELETE("/auth/google/link");
			if (error) throw { error, response };
			return;
		},
		onSuccess: refreshCurrentUser,
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

	const logout = async () => {
		const token = await getRefreshToken();
		if (!token) {
			router.replace("/(tabs)");
			return;
		}

		try {
			await client.POST("/auth/logout", {
				body: {
					refresh_token: token,
				},
			});
		} catch (e) {
			console.log(e);
		} finally {
			console.log("logout");
			await zustandLogout();
			router.replace("/(tabs)");
		}
	};

	return {
		sendCode,
		verifyCode,
		authGoogle,
		authGoogleLink,
		authGoogleUnlink,
		upsertEmail,
		logout,
		isProcessing:
			sendCode.isPending ||
			verifyCode.isPending ||
			authGoogle.isPending ||
			authGoogleLink.isPending ||
			authGoogleUnlink.isPending ||
			upsertEmail.isPending,
	};
};
